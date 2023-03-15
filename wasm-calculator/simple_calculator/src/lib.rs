//! A simple calculator implementation.
//!
//! An implementation of a calculator that features:
//! - DMAS (division, muliplication, addition, subtraction) operations.
//! - Memory of last calculation.
//! - Percentage conversion.
//! 
//! Example
//! ```rust
//! use simple_calculator::Calculator;
//! 
//! let calc = Calculator::new();
//! ```
mod calculator;
pub use calculator::Calculator;

mod operator;
pub use operator::Operator;

mod number;
mod truncate;