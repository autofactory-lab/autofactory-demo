use autofactory_demo::{slugify, to_roman, word_count};

#[test]
fn public_api_works_together() {
    let title = format!("Chapter {}", to_roman(3).unwrap());
    assert_eq!(slugify(&title), "chapter-iii");
    assert_eq!(word_count(&title), 2);
}
