use crate::result_output_html::TextMaybeBold;
use gcollections::ops::*;
use interval::interval_set::*;
use lazy_static::lazy_static;
use regex::Regex;

/// Extracts lists of consecutive words of list length n from the provided text.
///     Cleans the text first.
pub fn extract_clean_word_ngrams(words: &Vec<String>, n: usize) -> Vec<String> {
    let mut output = Vec::new();
    // No way to find plagiarism if chunk size > # words
    if n > words.len() {
        return Vec::new();
    }

    // Inclusive iterator between these two values
    for i in 0..=(words.len() - n) {
        let mut ngram = Vec::new();
        for j in 0..n {
            ngram.push(words[i + j].to_string());
        }
        output.push(ngram.join(" "));
    }
    output
}

/// Removes nonalphanumeric characters, redundant spaces, newlines,
///     converts to lowecase and trims text
pub fn clean_text(text: &str) -> Vec<String> {
    // Compile this only once
    lazy_static! {
        static ref REMOVE_NONALPHA: Regex = Regex::new(r"[^A-Za-z0-9 ]")
            .expect("Regex to remove non-alphanumeric characters could not be compiled properly!");
        static ref REMOVE_SPACES: Regex = Regex::new(r"\s+")
            .expect("Regex to remove redundant spaces could not be compiled properly!");
    }
    // Remove all newlines and convert to lowercase
    let mut new_text: String = text.replace('\n', " ").to_lowercase();
    // Remove all non alphanumeric+space chars and remove redundant spaces
    new_text = REMOVE_NONALPHA.replace_all(&new_text, " ").to_string();
    REMOVE_SPACES
        .replace_all(&new_text, " ")
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect()
}

/// Given a list of words and the intervals (union-ed) that are plagiarized:
///     Separate the words into text segments where plagiarized segments are indicated
///     in bold.
/// Algorithm:
///     - Since the intervals are sorted, we read each interval one by one from start
///     -
pub fn get_boldtext_segments_from_intervals(
    words: &Vec<String>,
    text_intervals: &IntervalSet<usize>,
) -> Vec<TextMaybeBold> {
    let mut text_segments: Vec<TextMaybeBold> = Vec::new();
    let mut cur_words: Vec<String> = Vec::new();
    let mut contains_previously = false;

    for (i, item) in words.iter().enumerate() {
        let word = item.clone();
        if text_intervals.contains(&i) {
            // In interval, should be bold
            if contains_previously {
                cur_words.push(word);
            } else {
                if !cur_words.is_empty() {
                    text_segments.push(TextMaybeBold {
                        text: cur_words.join(" "),
                        is_bold: false,
                    });
                }
                cur_words = Vec::new();
                cur_words.push(word);
            }
            contains_previously = true;
        } else {
            // Not in interval now
            if contains_previously {
                if !cur_words.is_empty() {
                    text_segments.push(TextMaybeBold {
                        text: cur_words.join(" "),
                        is_bold: true,
                    });
                }
                cur_words = Vec::new();
                cur_words.push(word);
            } else {
                cur_words.push(word);
            }
            contains_previously = false;
        }
    }
    if !cur_words.is_empty() {
        if contains_previously {
            text_segments.push(TextMaybeBold {
                text: cur_words.join(" "),
                is_bold: true,
            });
        } else {
            text_segments.push(TextMaybeBold {
                text: cur_words.join(" "),
                is_bold: false,
            });
        }
    }
    text_segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ngram() {
        assert_eq!(
            extract_clean_word_ngrams(
                &vec!["mary".to_string(), "had".to_string(), "a".to_string()],
                2
            ),
            vec!["mary had", "had a"]
        );

        assert_eq!(
            extract_clean_word_ngrams(&clean_text("    ||| mary\n  @@@@ ....  had a\n\n\n"), 2),
            vec!["mary had", "had a"]
        );
    }

    #[test]
    fn test_clean() {
        assert_eq!(
            clean_text("  a  b c \n 2 3 @ 4\n224acb@\n"),
            vec!["a", "b", "c", "2", "3", "4", "224acb"]
        )
    }

    #[test]
    fn test_intervals_firstwords_bold() {
        let words = vec!["a", "b", "c", "d", "e"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let intervals = vec![(0, 1)].to_interval_set();
        assert_eq!(
            get_boldtext_segments_from_intervals(&words, &intervals),
            vec![
                TextMaybeBold {
                    text: "a b".to_string(),
                    is_bold: true
                },
                TextMaybeBold {
                    text: "c d e".to_string(),
                    is_bold: false
                }
            ]
        );
    }

    #[test]
    fn test_intervals_lastwords_bold() {
        let words = vec!["a", "b", "c", "d", "e"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let intervals = vec![(2, 4)].to_interval_set();
        assert_eq!(
            get_boldtext_segments_from_intervals(&words, &intervals),
            vec![
                TextMaybeBold {
                    text: "a b".to_string(),
                    is_bold: false
                },
                TextMaybeBold {
                    text: "c d e".to_string(),
                    is_bold: true
                }
            ]
        );
    }

    #[test]
    fn test_intervals_no_bold() {
        let words = vec!["a", "b", "c", "d", "e"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let intervals = vec![].to_interval_set();
        assert_eq!(
            get_boldtext_segments_from_intervals(&words, &intervals),
            vec![TextMaybeBold {
                text: "a b c d e".to_string(),
                is_bold: false
            },]
        );
    }

    #[test]
    fn test_intervals_all_bold() {
        let words = vec!["a", "b", "c", "d", "e"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let intervals = vec![(0, 4)].to_interval_set();
        assert_eq!(
            get_boldtext_segments_from_intervals(&words, &intervals),
            vec![TextMaybeBold {
                text: "a b c d e".to_string(),
                is_bold: true
            },]
        );
    }

    #[test]
    fn test_intervals_single_letters_bold() {
        let words = vec!["a", "b", "c", "d", "e"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let intervals = vec![(0, 0), (2, 2)].to_interval_set();
        assert_eq!(
            get_boldtext_segments_from_intervals(&words, &intervals),
            vec![
                TextMaybeBold {
                    text: "a".to_string(),
                    is_bold: true
                },
                TextMaybeBold {
                    text: "b".to_string(),
                    is_bold: false
                },
                TextMaybeBold {
                    text: "c".to_string(),
                    is_bold: true
                },
                TextMaybeBold {
                    text: "d e".to_string(),
                    is_bold: false
                },
            ]
        );
    }
}
