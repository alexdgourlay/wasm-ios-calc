/// For truncating things.
pub trait Truncate {
    fn truncate_nums(&self, len: usize) -> &str;
}

impl Truncate for str {
    /// Truncate the number of numeric characters to a given maximum count.
    ///
    // TODO: round last digit.
    fn truncate_nums(&self, max_count: usize) -> &str {
        let mut numeric_char_count = 0;
        let mut slice_index = 0;

        for (index, char) in self.chars().enumerate() {
            slice_index = index;
            if char.is_digit(10) {
                numeric_char_count += 1;
            }
            if numeric_char_count == max_count {
                break;
            }
        }
        &self[0..slice_index + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_nums() {
        let test_str = "1234";
        assert_eq!(
            test_str.truncate_nums(2),
            "12",
            "Should reduce the number of numeric characters to the given count."
        );
    }

    #[test]
    fn truncate_nums_decimal() {
        let test_str = "1.234";
        assert_eq!(
            test_str.truncate_nums(2),
            "1.2",
            "Should reduce the number of numeric characters to the given count."
        );
    }

    #[test]
    fn truncate_nums_over_len() {
        let test_str = "1234";
        assert_eq!(test_str.truncate_nums(6), test_str, "Should have no effect when the max count is greater than the number of numeric characters.");
    }
}
