//! Text helpers. Public, so other crates may use them too.

/// Lowercased words with punctuation trimmed. Empty words are dropped.
pub fn words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(clean)
        .filter(|w| !w.is_empty())
        .collect()
}

// Private (no `pub`): only code in this module can call it.
// Python's `_clean` is a convention; this is enforced by the compiler.
fn clean(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*; // child modules can see their parent's private items

    #[test]
    fn clean_trims_punctuation() {
        assert_eq!(clean("\"Rust!\""), "rust");
    }

    #[test]
    fn words_drops_empty_tokens() {
        assert_eq!(words("-- hi ,"), ["hi"]);
    }
}
