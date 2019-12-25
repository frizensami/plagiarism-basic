use strsim::{hamming, levenshtein};
use crate::Metric;

pub fn is_plagiarised(s1: &str, s2: &str, metric: Metric, cutoff: usize) -> bool {
    match metric {
        Metric::Equal => check_equal(s1, s2),
        Metric::Hamming => check_hamming(s1, s2, cutoff),
        Metric::Lev => check_lev(s1, s2, cutoff)
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