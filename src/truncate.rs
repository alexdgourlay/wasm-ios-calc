pub trait Truncate {
    fn truncate_nums(&self, len: usize) -> &str;
}

impl Truncate for str {

    // TODO: round last digit.
    fn truncate_nums(&self, max_count: usize) -> &str {
        let mut num_count = 0;
        let mut slice_index = 0;

        for (i, char) in self.chars().enumerate() {
            slice_index = i;
            if char.is_digit(10) {
                num_count += 1;
            }
            if num_count == max_count {
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
        let test_str = "1.234";
        assert_eq!(test_str.truncate_nums(2), "1.2");
    }

    #[test]
    fn truncate_nums_over_len() {
        let test_str = "1.234";
        assert_eq!(test_str.truncate_nums(6), "1.234");
    }
}