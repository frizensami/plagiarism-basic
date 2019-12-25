use regex::Regex;

/// Extracts lists of consecutive words of list length n from the provided text.
///     Cleans the text first.
pub fn extract_clean_word_ngrams(text: &str, n: usize) -> Vec<String> {
    let cleaned_text = clean_text(text);
    let words : Vec<&str> = cleaned_text.split_whitespace().collect();
    let mut output = Vec::new();
    for i in 0..(words.len() - n + 1) {
        let mut ngram = Vec::new();
        for j in 0..n {
            ngram.push(words[i + j].to_string());
        }
        output.push(ngram.join(" "));
    }
    return output
}

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
    fn test_ngram() {
        assert_eq!(
            extract_clean_word_ngrams("mary had a", 2),
            vec!["mary had", "had a"]
        );

        assert_eq!(
            extract_clean_word_ngrams("    ||| mary\n  @@@@ ....  had a\n\n\n", 2),
            vec!["mary had", "had a"]
        );
    }

    #[test]
    fn test_clean() {
        assert_eq!(
            clean_text("  a  b c \n 2 3 @ 4\n224acb@\n"),
            "a b c 2 3 4 224acb"
        )
    }
}
