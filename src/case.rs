//! Letter case.

/// Capitalises the first letter of every whitespace-separated word.
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|w| format!("{}{}", w[..1].to_uppercase(), &w[1..]))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalises() {
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("rust is fun"), "Rust Is Fun");
    }
}
