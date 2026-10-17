//! Phase 01 · Lesson 07 — String vs &str, UTF-8 & text processing
//!
//! Run:  cargo run -p strings
//! Test: cargo test -p strings
//!
//! Scenario: cleaning text before it goes to a model: normalising whitespace,
//! truncating safely, and handling non-English text (Vietnamese here) correctly.

/// Collapse all runs of whitespace into single spaces and trim the ends.
/// Takes &str (borrows), returns String (a new owned value).
fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Truncate to at most `max_chars` characters, adding "…" if anything was cut.
/// Slicing by bytes (&text[..n]) would panic in the middle of a multi-byte character.
fn truncate_chars(text: &str, max_chars: usize) -> String {
    match text.char_indices().nth(max_chars) {
        None => text.to_string(), // already short enough
        Some((byte_idx, _)) => format!("{}…", &text[..byte_idx]),
    }
}

/// Borrow a sub-slice without copying: the first line of a text.
fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or("")
}

/// Redact anything that looks like an API key ("sk-" followed by letters/digits).
fn redact_keys(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for (i, word) in text.split(' ').enumerate() {
        if i > 0 {
            out.push(' ');
        }
        if word.starts_with("sk-") && word.len() > 10 {
            out.push_str("sk-***");
        } else {
            out.push_str(word);
        }
    }
    out
}

fn main() {
    // Two string types:
    let literal: &str = "Rust  is\tfast"; // borrowed, read-only, often static
    let owned: String = String::from("growable"); // owned, heap-allocated, mutable if `mut`
    let mut built = owned.clone();
    built.push_str(" string");
    built += "!";
    println!("&str          : {literal:?}");
    println!("String        : {built:?}");

    // &String coerces to &str, so functions taking &str accept both.
    println!("normalized    : {:?}", normalize_whitespace(literal));
    println!("normalized    : {:?}", normalize_whitespace(&built));

    // UTF-8: len() is bytes, not characters.
    let vi = "Xin chào thế giới";
    println!("len (bytes)   : {}", vi.len());
    println!("chars         : {}", vi.chars().count());
    println!("truncate 8    : {:?}", truncate_chars(vi, 8));
    println!("truncate 50   : {:?}", truncate_chars(vi, 50));
    println!("bytes[4..7]   : {:?}", vi.as_bytes().get(4..7));
    println!("char at 6     : {:?}", vi.chars().nth(6));

    // Searching and splitting
    let doc = "Title: Ownership\nOwnership means one owner.\nBorrowing lends access.";
    println!("first line    : {:?}", first_line(doc));
    println!("contains      : {}", doc.contains("Borrowing"));
    println!("find          : {:?}", doc.find("owner"));
    println!(
        "strip_prefix  : {:?}",
        first_line(doc).strip_prefix("Title: ")
    );
    let parts: Vec<&str> = "a,b,,c".split(',').collect();
    println!("split         : {parts:?}");

    // Formatting
    // {x:>10} right-align in 10, {x:<8.2} left-align with 2 decimals, {:05} zero-pad.
    let name = "chunker";
    let ms = 12.3456;
    let line = format!("{name:>10}|{ms:<8.2}|{:05}", 42);
    println!("format        : {line}");

    // Redaction before logging prompts
    println!(
        "redacted      : {}",
        redact_keys("use key sk-abc123XYZ456 for prod")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_collapses_newlines_and_tabs() {
        assert_eq!(normalize_whitespace("  a\n\n b\t c  "), "a b c");
    }

    #[test]
    fn truncate_never_splits_a_character() {
        assert_eq!(truncate_chars("thế giới", 2), "th…");
        assert_eq!(truncate_chars("thế giới", 3), "thế…");
        assert_eq!(truncate_chars("abc", 3), "abc");
    }

    #[test]
    fn first_line_of_empty_is_empty() {
        assert_eq!(first_line(""), "");
    }

    #[test]
    fn redact_keeps_other_words() {
        assert_eq!(redact_keys("a sk-1234567890ab b"), "a sk-*** b");
        assert_eq!(redact_keys("sk-short"), "sk-short");
    }
}
