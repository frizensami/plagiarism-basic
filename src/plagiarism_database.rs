use crate::string_compare::is_plagiarised;
use crate::text_utils::extract_clean_word_ngrams;
use crate::Metric;
use serde::Serialize;
use std::collections::HashSet;
use std::iter::FromIterator;

type TextOwnerID = String;

/// Report for plagiarism between two owners
#[derive(Serialize)]
pub struct PlagiarismResult {
    pub owner_id1: TextOwnerID,
    pub owner_id2: TextOwnerID,
    pub matching_fragments: Vec<(String, String)>,
    pub trusted_owner1: bool,  // Is the first owner a trusted source?
    pub equal_fragments: bool, // Can we ignore one element of the tuple?
}

/// A single user's "submission" or text string, broken into fragments
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
    pub fn check_untrusted_plagiarism(&self) -> Vec<PlagiarismResult> {
        let mut results: Vec<PlagiarismResult> = Vec::new();
        for i in 0..self.untrusted_texts.len() {
            for j in (i + 1)..self.untrusted_texts.len() {
                let source = &self.untrusted_texts[i];
                let against = &self.untrusted_texts[j];

                let matching_fragments = match self.metric {
                    Metric::Equal => self.check_plagiarism_equal(source, against),
                    _ => self.check_plagiarism_other(source, self.metric, against),
                };
                if matching_fragments.is_empty() {
                    continue;
                }
                let result = PlagiarismResult {
                    owner_id1: source.owner.clone(),
                    owner_id2: against.owner.clone(),
                    matching_fragments: matching_fragments,
                    trusted_owner1: false,
                    equal_fragments: self.metric == Metric::Equal,
                };
                results.push(result);
            }
        }
        results
    }

    /// Check for plagiarism by comparing metric against cutoff
    ///     for textfragments in database against trusted fragments
    pub fn check_trusted_plagiarism(&self) -> Vec<PlagiarismResult> {
        let mut results: Vec<PlagiarismResult> = Vec::new();
        println!("\n\nChecking against trusted sources...\n");
        for i in 0..self.trusted_texts.len() {
            for j in 0..self.untrusted_texts.len() {
                let source = &self.trusted_texts[i];
                let against = &self.untrusted_texts[j];
                let matching_fragments = match self.metric {
                    Metric::Equal => self.check_plagiarism_equal(source, against),
                    _ => self.check_plagiarism_other(source, self.metric, against),
                };
                if matching_fragments.is_empty() {
                    continue;
                }
                let result = PlagiarismResult {
                    owner_id1: source.owner.clone(),
                    owner_id2: against.owner.clone(),
                    matching_fragments: matching_fragments,
                    trusted_owner1: true,
                    equal_fragments: self.metric == Metric::Equal,
                };
                results.push(result);
            }
        }
        results
    }

    /// Splits a text string into separate ngram TextFragments
    fn get_textfragments(text: &str, n: usize) -> HashSet<String> {
        let ngrams = extract_clean_word_ngrams(text, n);
        HashSet::from_iter(ngrams)
    }

    /// Checks plagiarism by equality of fragments, uses fast set intersection
    /// Returns a tuple of all matches (second tuple element is identical to first)
    fn check_plagiarism_equal(
        &self,
        source: &TextEntry,
        against: &TextEntry,
    ) -> Vec<(String, String)> {
        let intersect: Vec<&String> = source.fragments.intersection(&against.fragments).collect();
        if !intersect.is_empty() {
            // Plagiarism!
            intersect
                .iter()
                .map(|val| (val.to_string(), val.to_string()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Checks plagiarism by non-equal metric (string-by-string)
    /// Returns a tuple of all matches (second tuple element is identical to first)
    fn check_plagiarism_other(
        &self,
        source: &TextEntry,
        metric: Metric,
        against: &TextEntry,
    ) -> Vec<(String, String)> {
        let mut results: Vec<(String, String)> = Vec::new();
        for source_frag in source.fragments.iter() {
            for against_frag in against.fragments.iter() {
                if is_plagiarised(source_frag, against_frag, metric, self.s) {
                    results.push((source_frag.to_string(), against_frag.to_string()));
                }
            }
        }
        results
    }
}
