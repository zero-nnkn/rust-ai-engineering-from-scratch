//! Phase 00 · Lesson 02 — the library half of the package.
//!
//! A Cargo package can hold one library (`src/lib.rs`) and any number of
//! binaries (`src/main.rs`, `src/bin/*.rs`). The library is where logic lives;
//! binaries are thin wrappers that parse input and print output. Same idea as a
//! Python package with a small `__main__.py`.

/// Turn a title into a URL/file-friendly slug: lowercase ASCII letters and digits,
/// with single dashes between words.
///
/// ```
/// assert_eq!(cargo_workflow::slugify("Hello, Rust World!"), "hello-rust-world");
/// ```
pub fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(ch.to_ascii_lowercase());
            pending_dash = false;
        } else {
            pending_dash = true;
        }
    }
    slug
}

/// Count the words in a text (split on any whitespace).
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_collapses_punctuation_and_spaces() {
        assert_eq!(slugify("  Rust -- for   AI  "), "rust-for-ai");
    }

    #[test]
    fn slugify_of_only_punctuation_is_empty() {
        assert_eq!(slugify("!!! ???"), "");
    }

    #[test]
    fn word_count_ignores_extra_whitespace() {
        assert_eq!(word_count("  agents \n are\tloops "), 3);
    }
}
