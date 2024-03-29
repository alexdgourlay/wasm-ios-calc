use super::truncate::Truncate;
use num_format::{Locale, ToFormattedString};
use std::{cmp, fmt::Display};

/// Represents a number.
///
/// Stores both a floating point value and string value of a number.
/// The string value permits leading and trailing zeroes to be displayed.
#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    // The floating point value of the number.
    value: f64,
    // The string value of the number.
    value_str: String,
    // The number of significant figures used for formatting.
    sf: Option<u8>,
}

impl Number {
    /// Returns the numeric value of the number.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Sets the value of the number.
    ///
    /// Updates both the numeric and string value.
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
        self.value_str = value.to_string();
    }

    /// Append a decimal point to the number if valid.
    pub fn decimalise(&mut self) {
        if self.value.fract() != 0. || self.value_str.ends_with('.') {
            return;
        }
        self.value_str.push('.');
    }

    /// Append a number onto the end of the number.
    pub fn append(&mut self, number: u8) {
        if let Some(sf) = self.sf {
            // Can't append a new number beyond the number of significant figures.
            if self.value_str.len() == sf.into() {
                return;
            }
        }
        if self.value_str == "0" {
            self.value_str = number.to_string();
        } else if self.value_str == "-0" {
            self.value_str = format!("-{}", number);
        } else {
            self.value_str.push_str(&number.to_string());
        }

        // Set the numeric value from the updated string value.
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
    /// Formats the number.
    ///
    /// TODO: localise format, for example French format uses comma as decimal point.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert to exponential notation.
        let exponential_format = format!("{:e}", self.value);

        // Split exponential into coefficient and exponential parts.
        let mut split = exponential_format.split('e');
        let coefficient_str = split.next().unwrap();
        let exponent_str = split.next().unwrap();

        // Parse exponent into float.
        let exponent: f64 = exponent_str.parse().unwrap();

        if let Some(sf) = self.sf {
            // If the exponent is sufficiently large or small, then output exponential notation.
            if exponent.abs() >= sf.into() {
                let exponent_str_len = exponent.to_string().len() as u8;

                // Calculate the number of digits that should be displayed in the coefficient.
                let max_coefficient_len = cmp::max(1, sf - (exponent_str_len + 1));

                let trunc_coefficient = coefficient_str.truncate_nums(max_coefficient_len.into());

                /* Exponential notation */
                return write!(f, "{}e{}", trunc_coefficient, exponent_str);
            }
        }

        // Formatted output string to be returned.
        let mut formatted_output = String::new();

        // Get integer component.
        let int = self.value().trunc() as i64;

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

#[cfg(test)]
mod tests {
    use super::Number;

    #[test]
    fn append() {
        let mut number = Number::from(1);
        number.append(2);
        assert_eq!(number.value, 12., "Should append number.");
    }

    #[test]
    fn append_decimal() {
        let mut number = Number::from(1);
        number.decimalise();
        number.append(2);
        assert_eq!(number.value, 1.2, "Should append number after decimal.");
    }

    #[test]
    fn decimalise() {
        let mut number = Number::from(1);
        number.decimalise();

        assert_eq!(number.value, 1., "Should not effect numeric value.");
        assert_eq!(number.to_string(), "1.", "Should add decimal to end of number string.");
    }

    #[test]
    fn formats_trailing_zeroes() {
        let mut number = Number::from(1);
        number.decimalise();
        number.append(0);
        assert_eq!(format!("{}", number), "1.0");
    }

    #[test]
    fn formats_thousands() {
        let number = Number::from(1234.56);
        assert_eq!(format!("{}", number), "1,234.56");
    }

    #[test]
    fn formats_millions() {
        let number = Number::from(1234567.89);
        assert_eq!(format!("{}", number), "1,234,567.89");
    }

    #[test]
    fn formats_large_exp_a() {
        let number = Number::from(1234567890.);
        assert_eq!(format!("{}", number), "1.234567e9");
    }

    #[test]
    fn formats_large_exp_b() {
        let number = Number::from(12345678900.);
        assert_eq!(format!("{}", number), "1.23456e10");
    }

    #[test]
    fn formats_large_neg_exp() {
        let number = Number::from(-1234567890.);
        assert_eq!(format!("{}", number), "-1.234567e9");
    }

    #[test]
    fn formats_small_exp() {
        let number = Number::from(0.00000000123456789);
        assert_eq!(format!("{}", number), "1.23456e-9");
    }

    #[test]
    fn formats_small_neg_exp() {
        let number = Number::from(-0.00000000123456789);
        assert_eq!(format!("{}", number), "-1.23456e-9");
    }
}
