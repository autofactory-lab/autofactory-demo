//! Roman numerals.

const SYMBOLS: [(u32, &str); 7] = [
    (1000, "M"),
    (500, "D"),
    (100, "C"),
    (50, "L"),
    (10, "X"),
    (5, "V"),
    (1, "I"),
];

/// Converts `n` (1..=3999) to a Roman numeral. Returns `None` outside that range.
pub fn to_roman(n: u32) -> Option<String> {
    if !(1..=3999).contains(&n) {
        return None;
    }
    let mut rest = n;
    let mut out = String::new();
    for (value, symbol) in SYMBOLS {
        while rest >= value {
            out.push_str(symbol);
            rest -= value;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_numbers() {
        assert_eq!(to_roman(1).as_deref(), Some("I"));
        assert_eq!(to_roman(3).as_deref(), Some("III"));
        assert_eq!(to_roman(5).as_deref(), Some("V"));
        assert_eq!(to_roman(8).as_deref(), Some("VIII"));
        assert_eq!(to_roman(10).as_deref(), Some("X"));
        assert_eq!(to_roman(2026).as_deref(), Some("MMXXVI"));
    }

    #[test]
    fn out_of_range() {
        assert_eq!(to_roman(0), None);
        assert_eq!(to_roman(4000), None);
    }
}
