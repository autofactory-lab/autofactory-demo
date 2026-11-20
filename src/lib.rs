//! Small text utilities used to demonstrate an automated maintenance workflow.

pub mod roman;
pub mod slug;
pub mod words;

pub use roman::to_roman;
pub use slug::slugify;
pub use words::{truncate_words, word_count};
