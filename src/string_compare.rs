use crate::Metric;
use strsim::levenshtein;

pub fn is_plagiarised(s1: &str, s2: &str, metric: Metric, cutoff: usize) -> bool {
    match metric {
        Metric::Equal => check_equal(s1, s2),
        Metric::Lev => check_lev(s1, s2, cutoff),
    }
}

/// Checks if two strings are equal (helper function for SLAP)
#[inline]
fn check_equal(s1: &str, s2: &str) -> bool {
    s1 == s2
}

/// Check levenshtein distance between strings against cutoff
fn check_lev(s1: &str, s2: &str, cutoff: usize) -> bool {
    levenshtein(s1, s2) <= cutoff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert!(is_plagiarised("a", "a", Metric::Equal, 0));
    }

    #[test]
    fn test_lev() {
        assert!(is_plagiarised("abcd", "ac", Metric::Lev, 2));
        assert!(!is_plagiarised("abcd", "ac", Metric::Lev, 1));
    }
}
