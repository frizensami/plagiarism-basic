use crate::string_compare::is_plagiarised;
use crate::text_utils::extract_clean_word_ngrams;
use crate::Metric;
use std::collections::HashSet;
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
            for j in (i + 1)..self.untrusted_texts.len() {
                let source = &self.untrusted_texts[i];
                let against = &self.untrusted_texts[j];
                match self.metric {
                    Metric::Equal => self.check_plagiarism_equal(source, against),
                    _ => self.check_plagiarism_other(source, self.metric, against),
                }
            }
        }
    }

    /// Splits a text string into separate ngram TextFragments
    fn get_textfragments(text: &str, n: usize) -> HashSet<String> {
        let ngrams = extract_clean_word_ngrams(text, n);
        HashSet::from_iter(ngrams)
    }

    fn check_plagiarism_equal(&self, source: &TextEntry, against: &TextEntry) {
        let intersect: Vec<&String> = source.fragments.intersection(&against.fragments).collect();
        if !intersect.is_empty() {
            // Plagiarism!
            println!(
                "Detected plagiarism between {} and {}! Detected similarities: ",
                source.owner, against.owner
            );
            println! {"{:?}", intersect}
        }
    }

    fn check_plagiarism_other(&self, source: &TextEntry, metric: Metric, against: &TextEntry) {
        for source_frag in source.fragments.iter() {
            for against_frag in against.fragments.iter() {
                if is_plagiarised(source_frag, against_frag, metric, self.s) {
                    println!(
                        "Detected plagiarism between {} and {}! Detected similarity: \n",
                        source.owner, against.owner
                    );
                    println!("Fragment 1: {}\nFragment 2: {}\n\n", source_frag, against_frag)
                }
            }
        }
    }
}
