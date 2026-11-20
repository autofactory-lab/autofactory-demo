//! Small text utilities used to demonstrate an automated maintenance workflow.

pub mod case;
pub mod roman;
pub mod slug;
pub mod words;

pub use case::title_case;
pub use roman::to_roman;
pub use slug::slugify;
pub use words::word_count;
