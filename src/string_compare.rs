use crate::Metric;
use strsim::{hamming, levenshtein};

pub fn is_plagiarised(s1: &str, s2: &str, metric: Metric, cutoff: usize) -> bool {
    match metric {
        Metric::Equal => check_equal(s1, s2),
        Metric::Hamming => check_hamming(s1, s2, cutoff),
        Metric::Lev => check_lev(s1, s2, cutoff),
    }
}

/// Checks if two strings are equal (helper function for SLAP)
#[inline]
fn check_equal(s1: &str, s2: &str) -> bool {
    s1 == s2
}

/// Assumes that strings are equal length
///  and checks hamming distance between them against cutoff
fn check_hamming(s1: &str, s2: &str, cutoff: usize) -> bool {
    hamming(s1, s2).unwrap() > cutoff
}

/// Check levenshtein distance between strings against cutoff
fn check_lev(s1: &str, s2: &str, cutoff: usize) -> bool {
    levenshtein(s1, s2) > cutoff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert!(is_plagiarised("a", "a", Metric::Equal, 0));
    }

    #[test]
    fn test_hamming() {
        assert!(is_plagiarised("abc", "acb", Metric::Hamming, 1));
        assert!(is_plagiarised("abcd", "accf", Metric::Hamming, 1));
    }

    #[test]
    #[should_panic]
    fn test_hamming_panic() {
        assert!(is_plagiarised("not", "equal_length", Metric::Hamming, 1));
    }

    #[test]
    fn test_lev() {
        assert!(is_plagiarised("abcd", "ac", Metric::Lev, 1));
    }
}
