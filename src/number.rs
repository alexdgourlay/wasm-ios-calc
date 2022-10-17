use crate::truncate::Truncate;
use num_format::{Locale, ToFormattedString};
use std::{cmp, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    value: f64,
    pub value_str: String,
    sf: Option<u8>,
}

impl Number {
    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
        self.value_str = value.to_string();
    }

    pub fn decimalise(&mut self) {
        if self.value.fract() != 0. || self.value_str.ends_with('.') {
            return;
        }
        self.value_str.push('.');
    }

    pub fn concat(&mut self, number: u8) {
        if let Some(sf) = self.sf {
            // Can't concatinate a new number beyond the number of significant figures.
            if self.value_str.len() == sf.into() {
                return;
            }
        }
        if self.value_str == "0" {
            self.value_str = number.to_string();
        } else {
            self.value_str.push_str(&number.to_string());
        }
        self.value = self.value_str.parse().unwrap();
    }
}

impl<T: Into<f64> + Display + Copy> From<T> for Number {
    fn from(value: T) -> Self {
        Number {
            value: value.into(),
            value_str: value.to_string(),
            sf: Some(9),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(sf) = self.sf {
            // If above the large number limit, display in exponential notation.
            if self.value.abs() >= 10f64.powi(sf.into()) {
                let exponent = self.value.log10().abs() as i32;

                let exponent_str_len = exponent.to_string().len() as u8;

                let coefficient = self.value / 10f64.powi(exponent);

                // Remove length of exponent string from sig figs e.g. "e1" has length of 2.
                let new_sf = cmp::max(1, sf - (exponent_str_len + 1));

                let mut trunc_coefficient = &*coefficient.to_string();

                // Truncate the output
                trunc_coefficient = trunc_coefficient.truncate_nums(new_sf.into());

                // Remove trailing zeroes.
                trunc_coefficient = trunc_coefficient.trim_end_matches('0');

                // Remove trailing decimal
                if trunc_coefficient.ends_with('.') {
                    trunc_coefficient =
                        &trunc_coefficient[0..trunc_coefficient.len() - 1];
                }

                return write!(f, "{}e{}", trunc_coefficient, exponent.to_string(),);
            }
        }

        // Formatted output string to be returned.
        let mut formatted_output = String::new();

        // Get integer component.
        let int = self.get_value().trunc() as i64;

        // Edge case where zero is negative, preserve negative sign.
        if int == 0 && self.value.is_sign_negative() {
            formatted_output.push('-');
        }

        // Push formatted integer component.
        formatted_output.push_str(&int.to_formatted_string(&Locale::en).to_string());

        // Split string at the decimal point.
        let split: Vec<&str> = self.value_str.split('.').collect();

        // Push formatted fractional component.
        if let Some(fract) = split.get(1) {
            formatted_output.push('.');
            formatted_output.push_str(fract);
        }

        // Truncate string
        if let Some(sf) = self.sf {
            return write!(f, "{}", formatted_output.truncate_nums(sf.into()));
        }

        write!(f, "{}", formatted_output)
    }
}
