/// Represents a mathematical operator.
#[derive(Debug, PartialEq, Clone)]
pub struct Operator {
    /// The character identifier of the operator.
    pub id: char,
    /// The operator's mathematical function.
    pub function: fn(f64, f64) -> f64,
    /// The BIDMAS order of operation.
    order: usize,
}

impl Operator {
    /// Checks if the operator come after another in the BIDMAS order of operations.
    ///
    /// Example
    /// ```rust
    /// use simple_calculator::Operator;
    ///
    /// let add = Operator::try_from("+").unwrap();
    /// let multiply = Operator::try_from("*").unwrap();
    ///
    /// assert!(add.after(&multiply), "Addition comes after multiplication.");
    /// ```
    pub fn after(&self, other: &Self) -> bool {
        self.order > other.order
    }
}

impl TryFrom<&str> for Operator {
    type Error = &'static str;

    /// Create an operator from its string identifier.
    ///
    /// Example
    /// ```rust
    /// use simple_calculator::Operator;
    ///
    /// let add = Operator::try_from("+");
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "/" => Ok(Self {
                id: '/',
                function: |x, y| x / y,
                order: 2,
            }),
            "*" => Ok(Self {
                id: '*',
                function: |x, y| x * y,
                order: 2,
            }),
            "-" => Ok(Self {
                id: '-',
                function: |x, y| x - y,
                order: 3,
            }),
            "+" => Ok(Self {
                id: '+',
                function: |x, y| x + y,
                order: 3,
            }),
            _ => Err("Unknown operator."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Operator;

    #[test]
    fn after_true() {
        let add = Operator::try_from("+").unwrap();
        let multiply = Operator::try_from("*").unwrap();

        assert!(add.after(&multiply), "Addition comes after multiplication.");
    }

    #[test]
    fn after_false() {
        let add = Operator::try_from("+").unwrap();
        let multiply = Operator::try_from("*").unwrap();

        assert!(
            !multiply.after(&add),
            "Multiplication does not come after addition."
        );
    }
}
