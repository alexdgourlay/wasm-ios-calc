use crate::{number::Number, operator::Operator};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(Number),
    Operator(Operator),
}

pub struct Calculator {
    pub buffer: Vec<Token>,
    pub display_index: usize,
    cleared: bool,
    editing: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            buffer: Vec::from([Token::Number(Number::from(0.))]),
            display_index: 0,
            cleared: true,
            editing: false,
        }
    }

    pub fn cleared(&self) -> bool {
        self.cleared
    }

    pub fn clear(&mut self) {
        if self.cleared {
            // All clear
            *self = Calculator::new();
        } else {
            // Clear
            self.display_index = 0;
            self.buffer[0] = Token::Number(Number::from(0.));
            self.cleared = true;
        }
    }

    pub fn output(&mut self) -> &mut Number {
        match self.buffer.get_mut(self.display_index).unwrap() {
            Token::Number(number) => return number,
            _ => {
                panic!("Display index points to a non-number")
            }
        }
    }

    fn evaluate(&self) -> Result<Number, &str> {
        let mut values = self.buffer.clone();
        let mut index = values.len() - 1;

        // Iterate through values in reverse to calculate new result.
        while index >= 2 {
            if let Some(Token::Number(number_b)) = values.get(index) {
                if let Some(Token::Operator(operator)) = values.get(index - 1) {
                    if let Some(Token::Number(number_a)) = values.get(index - 2) {
                        values[index - 2] = Token::Number(Number::from((operator.function)(
                            number_a.get_value(),
                            number_b.get_value(),
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

    pub fn calculate(&mut self) {
        self.editing = false;

        // Nothing to calculate.
        if self.buffer.len() <= 2 {
            return;
        }

        if let Ok(result) = self.evaluate() {
            // Update result.
            self.buffer[0] = Token::Number(result);
            // Display result.
            self.display_index = 0;

            if self.buffer.len() > 3 {
                self.buffer.drain(1..3);
            }
        }
    }

    pub fn active_operator(&self) -> Option<&Operator> {
        if let Token::Operator(operator) = self.buffer.last().unwrap() {
            return Some(operator);
        }
        return None;
    }

    fn last_operator(&self) -> Option<Operator> {
        let mut values_clone = self.buffer.clone();
        values_clone.reverse();
        values_clone.iter().find_map(|item| {
            if let Token::Operator(operator) = item {
                return Some(operator.to_owned());
            }
            None
        })
    }

    pub fn submit_operator(&mut self, operator: Operator) {
        self.editing = false;

        match self.buffer.last_mut() {
            Some(Token::Operator(active_operator)) => {
                if active_operator.id != operator.id {
                    *active_operator = operator;
                }
            }
            Some(Token::Number(..)) => {
                if self.display_index > 0 {
                    // If there is already an operator in the buffer.
                    if let Some(prev_operator) = self.last_operator() {
                        // If the last operator is before in the order of operations (BIDMAS).
                        if prev_operator.before(&operator) {
                            self.buffer.push(Token::Operator(operator));
                            return;
                        }
                    }
                    self.calculate();
                }
                self.buffer.drain(1..self.buffer.len());
                self.buffer.push(Token::Operator(operator));
            }
            None => panic!("Buffer is empty."),
        }
    }

    pub fn submit_number(&mut self, number: u8) {
        match self.buffer.last() {
            Some(Token::Number(..)) => {
                if self.editing {
                    self.output().concat(number);
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

    pub fn submit_decimal(&mut self) {
        if !self.editing {
            self.submit_number(0);
        }
        self.output().decimalise();
    }

    pub fn submit_negative(&mut self) {
        let output = self.output();
        output.set_value(output.get_value() * -1.);
        self.editing = true;
    }

    pub fn submit_percentage(&mut self) {
        if self.display_index > 0 {
            self.calculate();
        }
        let output = self.output();
        output.set_value(output.get_value() / 100.);
    }
}
