mod plagiarism_database;
mod string_compare;
mod text_utils;

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
    Lev,
}

const TEXT1: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum";

const TEXT2: &str = "enim ad minim veniam, quis nostrud";

fn main() {}
