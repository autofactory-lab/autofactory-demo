//! Word counting.

/// Counts whitespace-separated words.
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Keeps the first `max` words of `s`, joined by single spaces, and appends "…" when words were dropped.
pub fn truncate_words(s: &str, max: usize) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.len() <= max {
        return words.join(" ");
    }
    let mut out = words[..max].join(" ");
    out.push('…');
    out
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

    #[test]
    fn truncates() {
        assert_eq!(truncate_words("one two three", 5), "one two three");
        assert_eq!(truncate_words("one two three", 2), "one two…");
        assert_eq!(truncate_words("  spaced   out  ", 2), "spaced out");
        assert_eq!(truncate_words("anything", 0), "…");
    }
}
