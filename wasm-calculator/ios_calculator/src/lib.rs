#![warn(missing_docs)]
//! # A simple iOS calculator implementation.
//!
//! The functionality of the calculator is based on that of the iOS calculator and features:
//! - DMAS (division, muliplication, addition, subtraction) operations.
//! - Memory of last calculation.
//! - Percentage conversion.
//!
//! Example
//! ```rust
//! use ios_calculator::Calculator;
//!
//! let calc = Calculator::new();
//! ```
mod calculator;
pub use calculator::Calculator;

mod operator;
pub use operator::Operator;

mod number;
mod truncate;
