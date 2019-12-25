use crate::text_utils::extract_clean_word_ngrams;
use crate::Metric;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type TextOwnerID = String;

struct TextEntry {
    owner: TextOwnerID,
    fragments: HashSet<String>,
}

/// Stores the corpus of trusted and untrusted strings
pub struct PlagiarismDatabase {
    // Constant value for ngram size
    n: usize,
    // Constant value for metric cutoff value
    s: usize,
    // Metric to use
    metric: Metric,
    /// Map
    trusted_texts: Vec<TextEntry>,
    ///
    untrusted_texts: Vec<TextEntry>,
}

impl PlagiarismDatabase {
    /// Initializes the plagiarism sensitivity and similarity metric values
    ///     and the actual metric type to be used in computing plagiarism
    ///     scores
    pub fn new(n: usize, s: usize, metric: Metric) -> PlagiarismDatabase {
        PlagiarismDatabase {
            n,
            s,
            metric,
            trusted_texts: Vec::new(),
            untrusted_texts: Vec::new(),
        }
    }

    /// Adds a text string as potential plagiarism source material
    pub fn add_trusted_text(&mut self, owner_id: String, text: &str) {
        self.trusted_texts.push(TextEntry {
            owner: owner_id,
            fragments: PlagiarismDatabase::get_textfragments(text, self.n),
        });
    }

    // Adds a text string as a potential plagiarized string
    pub fn add_untrusted_text(&mut self, owner_id: String, text: &str) {
        self.untrusted_texts.push(TextEntry {
            owner: owner_id,
            fragments: PlagiarismDatabase::get_textfragments(text, self.n),
        });
    }

    /// Check for plagiarism by comparing metric against cutoff
    ///     for all textfragments currently in database
    pub fn check_untrusted_plagiarism(&self) {
        for i in 0..self.untrusted_texts.len() {
            for j in (i+1)..self.untrusted_texts.len() {
                let source = &self.untrusted_texts[i];
                let against = &self.untrusted_texts[j];
                let intersect: Vec<&String> =
                    source.fragments.intersection(&against.fragments).collect();
                if !intersect.is_empty() {
                    // Plagiarism!
                    println!("Detected plagiarism between {} and {}! Detected similarities: ", source.owner, against.owner);
                    println!{"{:?}", intersect}
                }
            }
        }
    }

    /// Splits a text string into separate ngram TextFragments
    fn get_textfragments(text: &str, n: usize) -> HashSet<String> {
        let ngrams = extract_clean_word_ngrams(text, n);
        HashSet::from_iter(ngrams)
    }
}
