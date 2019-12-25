use crate::text_utils::extract_clean_word_ngrams;
use crate::Metric;
use fasthash::murmur;
use std::collections::HashMap;

type TextOwnerID = String;

struct TextFragment {
    fragment: String,
    fragment_hash: u32,
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
    trusted_texts: HashMap<TextOwnerID, Vec<TextFragment>>,
    ///
    untrusted_texts: HashMap<TextOwnerID, Vec<TextFragment>>,
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
            trusted_texts: HashMap::new(),
            untrusted_texts: HashMap::new(),
        }
    }

    /// Adds a text string as potential plagiarism source material
    pub fn add_trusted_text(&mut self, owner_id: String, text: &str) {
        self.trusted_texts.insert(
            owner_id,
            PlagiarismDatabase::get_textfragments(text, self.n),
        );
    }

    // Adds a text string as a potential plagiarized string
    pub fn add_untrusted_text(&mut self, owner_id: String, text: &str) {
        self.untrusted_texts.insert(
            owner_id,
            PlagiarismDatabase::get_textfragments(text, self.n),
        );
    }

    /// Splits a text string into separate ngram TextFragments 
    fn get_textfragments(text: &str, n: usize) -> Vec<TextFragment> {
        let ngrams = extract_clean_word_ngrams(text, n);
        ngrams
            .iter()
            .map(|ngram| TextFragment {
                fragment: ngram.to_string(),
                fragment_hash: murmur::hash32(ngram),
            })
            .collect()
    }
}
