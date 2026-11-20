//! Word counting.

/// Counts whitespace-separated words.
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts() {
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("one"), 1);
        assert_eq!(word_count("  two words "), 2);
        assert_eq!(word_count("tabs\tand\nnewlines"), 3);
    }
}
