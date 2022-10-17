#[derive(Debug, PartialEq, Clone)]
pub struct Operator {
    pub id: String,
    pub function: fn(f64, f64) -> f64,
    order: usize,
}

impl Operator {
    pub fn before(&self, other: &Self) -> bool {
        self.order > other.order
    }
}

impl TryFrom<&str> for Operator {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "/" => Ok(Self {
                id: value.to_string(),
                function: |x, y| x / y,
                order: 2,
            }),
            "*" => Ok(Self {
                id: value.to_string(),
                function: |x, y| x * y,
                order: 2,
            }),
            "-" => Ok(Self {
                id: value.to_string(),
                function: |x, y| x - y,
                order: 3,
            }),
            "+" => Ok(Self {
                id: value.to_string(),
                function: |x, y| x + y,
                order: 3,
            }),
            _ => Err("Unknown operator.")
        }
    }
}