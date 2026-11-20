//! URL slugs.

/// Lowercases `s`, keeps ASCII letters and digits, and joins the rest with single dashes.
pub fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            if dash && !out.is_empty() {
                out.push('-');
            }
            dash = false;
            out.push(c.to_ascii_lowercase());
        } else {
            dash = true;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("  many   spaces  "), "many-spaces");
        assert_eq!(slugify("Rust 2021 Edition"), "rust-2021-edition");
        assert_eq!(slugify(""), "");
    }
}
