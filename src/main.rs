mod string_compare;
mod text_utils;
mod text_database;

use string_compare::is_plagiarised;

/// Overall strategy:
///     Take all input texts and for each:
///         - Remove newlines
///         - Convert to lowercase
///         - Trim
///         - Replace all non-alphanumeric characters with spaces
///         - use str::SplitWhitespace to get only non-whitespace words
///         - collect into vector
///         - take all l-length word sequences and join them
///         - Hash all these word sequences and provide those as well

pub enum Metric {
    Equal,
    Hamming,
    Lev
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert!(is_plagiarised("a", "a", Metric::Equal, 0));
    }
}