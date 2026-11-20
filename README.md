# autofactory-demo

A tiny Rust crate (`to_roman`, `slugify`, `word_count`) used to demonstrate an automated
maintenance workflow with [AutoCraft](https://github.com/storytold/autocraft): triage, review,
fixes, verification and merges.

```sh
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The code is intentionally small and contains known issues tracked in the issue list.
