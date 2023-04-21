use crate::{number::Number, operator::Operator};

/// A token that can be entered into the Calculator buffer.
///
/// Can be one of:
/// - [`Number`] - A representation of a number.
/// - [`Operator`] - A representation of a mathematical operator.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(Number),
    Operator(Operator),
}

/// A simple calulator.
pub struct Calculator {
    /// An vector of [`Token`] that can be parsed to calulate the output.
    buffer: Vec<Token>,
    /// The index into the buffer.
    display_index: usize,
    /// Is the calulcator's buffer cleared?
    cleared: bool,
    /// Is the calculator in an editing state?
    editing: bool,
}

impl Calculator {
    /// Create a new calculator.
    pub fn new() -> Self {
        Calculator {
            buffer: Vec::from([Token::Number(Number::from(0.))]),
            display_index: 0,
            cleared: true,
            editing: false,
        }
    }

    /// Getter function for the cleared property.
    pub fn cleared(&self) -> bool {
        self.cleared
    }

    /// Clears the buffer.
    ///
    /// Will perform a deep 'All clear' (AC) operation if the calulator is currently
    /// cleared.
    pub fn clear(&mut self) {
        if self.cleared {
            // All clear.
            *self = Calculator::new();
        } else {
            // Clear.
            self.display_index = 0;
            self.buffer[0] = Token::Number(Number::from(0.));
            self.cleared = true;
        }
    }

    /// Returns the [`Number`] output to be displayed.
    pub fn output(&mut self) -> &mut Number {
        match self.buffer.get_mut(self.display_index).unwrap() {
            Token::Number(number) => return number,
            _ => {
                panic!("Display index points to a non-number")
            }
        }
    }

    /// Resolves the calculation stored in the buffer.
    fn calculate(&self) -> Result<Number, &str> {
        let mut values = self.buffer.clone();
        let mut index = values.len() - 1;

        // Iterate through values in reverse order to calculate new result.
        while index >= 2 {
            if let Some(Token::Number(number_b)) = values.get(index) {
                if let Some(Token::Operator(operator)) = values.get(index - 1) {
                    if let Some(Token::Number(number_a)) = values.get(index - 2) {
                        values[index - 2] = Token::Number(Number::from((operator.function)(
                            number_a.value(),
                            number_b.value(),
                        )))
                    }
                }
            }
            index -= 2;
        }
        if let Some(Token::Number(result)) = values.first() {
            return Ok(result.to_owned());
        }
        Err("")
    }

    /// Returns the operator that is currently active if it exists.
    ///
    /// An operator is active if it is in the last place of the buffer
    /// and is therefore not succeeded by a [`Number`].
    pub fn active_operator(&self) -> Option<&Operator> {
        if let Some(Token::Operator(operator)) = self.buffer.last() {
            return Some(operator);
        }
        return None;
    }

    /// Returns the last entered operator in the buffer if one exists.
    fn last_operator(&self) -> Option<Operator> {
        // Iterate buffer in reverse order.
        for index in (0..self.buffer.len()).rev() {
            if let Some(Token::Operator(operator)) = self.buffer.get(index) {
                return Some(operator.to_owned());
            }
        }
        return None;
    }

    /// Performs the 'equals' operation.
    ///
    /// This will resolve the calculation stored in the buffer.
    pub fn submit_equals(&mut self) {
        self.editing = false;

        // Nothing to calculate.
        if self.buffer.len() <= 2 {
            return;
        }

        if let Ok(result) = self.calculate() {
            // Update result.
            self.buffer[0] = Token::Number(result);
            // Display result.
            self.display_index = 0;

            if self.buffer.len() > 3 {
                self.buffer.drain(1..3);
            }
        }
    }

    /// Submit an [`Operator`].
    pub fn submit_operator(&mut self, operator: Operator) {
        self.editing = false;

        match self.buffer.last_mut() {
            Some(Token::Operator(active_operator)) => {
                // Replace the existing active operator with the new operator.
                if active_operator.id != operator.id {
                    *active_operator = operator;
                }
            }
            Some(Token::Number(..)) => {
                if self.display_index > 0 {
                    // If there is already an operator in the buffer.
                    if let Some(prev_operator) = self.last_operator() {
                        // If the last is after in the order of operations (BIDMAS).
                        if prev_operator.after(&operator) {
                            self.buffer.push(Token::Operator(operator));
                            return;
                        }
                    }
                    self.submit_equals();
                }
                self.buffer.drain(1..self.buffer.len());
                self.buffer.push(Token::Operator(operator));
            }
            None => panic!("Buffer is empty."),
        }
    }

    /// Submit a number.
    ///
    /// If editing, this will append the number onto the number currently
    /// being edited.
    pub fn submit_number(&mut self, number: u8) {
        match self.buffer.last() {
            Some(Token::Number(..)) => {
                if self.editing {
                    self.output().append(number);
                } else {
                    self.buffer[0] = Token::Number(Number::from(number));
                }
            }
            Some(Token::Operator(..)) => {
                self.buffer.push(Token::Number(Number::from(number)));
                self.display_index += 2;
            }
            None => panic!("Buffer is empty."),
        };
        self.editing = true;
        self.cleared = false;
    }

    /// Performs the 'decimalise' operation.
    pub fn submit_decimal(&mut self) {
        if !self.editing {
            // Entering a decimal is handled as equivalent to submitting
            // a zero when not editing.
            self.submit_number(0);
        }
        self.output().decimalise();
    }

    /// Performs the 'negative' operation.
    pub fn submit_negative(&mut self) {
        let output = self.output();
        output.set_value(output.value() * -1.);
        self.editing = true;
    }

    /// Performs the 'percentage' operation.
    pub fn submit_percentage(&mut self) {
        if self.display_index > 0 {
            let _ = self.calculate();
        }
        let output = self.output();
        output.set_value(output.value() / 100.);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Calculator used for testing.
    fn test_calculator() -> Calculator {
        Calculator {
            display_index: 0,
            cleared: true,
            editing: false,
            buffer: vec![],
        }
    }

    #[test]
    fn last_operator_some() {
        let subtract = Operator::try_from("-").unwrap();
        let add = Operator::try_from("+").unwrap();
        let mut calc = test_calculator();
        calc.buffer = vec![
            Token::Operator(subtract.clone()),
            Token::Operator(add.clone()),
        ];

        assert_eq!(
            calc.last_operator(),
            Some(add),
            "Should return some last operator in the buffer."
        );
    }

    #[test]
    fn last_operator_none() {
        let number_1 = Number::from(1.);
        let mut calc = test_calculator();
        calc.buffer = vec![Token::Number(number_1.clone())];

        assert_eq!(
            calc.last_operator(),
            None,
            "Should return none when no operator in the buffer."
        );
    }

    #[test]
    fn last_operator_empty() {
        let calc = test_calculator();

        assert_eq!(
            calc.last_operator(),
            None,
            "Should return none when buffer is empty."
        );
    }

    #[test]
    fn submit_percentage() {
        let number_1 = Number::from(1.);
        let mut calc = test_calculator();
        calc.buffer = vec![Token::Number(number_1.clone())];
        calc.submit_percentage();

        assert_eq!(
            calc.output(),
            &Number::from(0.01),
            "Should convert output to a percentage."
        );
    }

    #[test]
    fn submit_percentage_negative() {
        let number_1 = Number::from(-1.);
        let mut calc = test_calculator();
        calc.buffer = vec![Token::Number(number_1.clone())];
        calc.submit_percentage();

        assert_eq!(
            calc.output(),
            &Number::from(-0.01),
            "Should convert output to a percentage."
        );
    }
}
