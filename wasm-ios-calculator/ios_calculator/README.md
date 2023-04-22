# Rusty iOS Calculator

[![Latest Version]][crates.io]
[crates.io]: https://crates.io/crates/ios_calculator

An implementation of the basic iOS calculator featuring:
- DMAS (division, muliplication, addition, subtraction) operations
- Decimalisation
- Memory of last calculation
- Percentage conversion

### Example
```rust
use ios_calculator::{Calculator, Operator};

let mut calc = Calculator::new();

calc.submit_number(1);
calc.submit_decimal();
calc.submit_number(2);
calc.submit_operator(Operator::add());
calc.submit_number(2);
calc.submit_equals();

assert_eq!(
    calc.output().value(),
    3.2,
);

assert_eq!(
    calc.output().to_string(),
    "3.2",
); 
```
