use regex::Regex;

pub fn extract_word_ngrams(text: &str) {}

fn clean_text(text: &str) -> String {
    let remove_nonalpha = Regex::new(r"[^A-Za-z0-9 ]").unwrap();
    let remove_spaces = Regex::new(r"\s+").unwrap();
    // Remove all newlines and convert to lowercase
    let mut new_text: String = text.replace('\n', " ").to_lowercase();
    // Remove all non alphanumeric+space chars and remove redundant spaces
    new_text = remove_nonalpha.replace_all(&new_text, " ").to_string();
    remove_spaces.replace_all(&new_text, " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean() {
        assert_eq!(clean_text("  a  b c \n 2 3 @ 4\n224acb@\n"), "a b c 2 3 4 224acb")
    }
}
