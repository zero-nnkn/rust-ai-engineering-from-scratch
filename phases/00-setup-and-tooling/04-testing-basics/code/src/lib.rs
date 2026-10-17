//! Phase 00 · Lesson 04 — Unit, integration & doc tests
//!
//! Test: cargo test -p testing-basics
//!
//! The code under test is a text chunker, the first step of any RAG pipeline:
//! split a document into chunks of at most `max_words` words, with `overlap`
//! words repeated between neighbouring chunks.

/// Why a chunking request was rejected.
#[derive(Debug, PartialEq)]
pub enum ChunkError {
    /// `max_words` was zero.
    ZeroSize,
    /// `overlap` must be smaller than `max_words`, or chunking never advances.
    OverlapTooLarge { overlap: usize, max_words: usize },
}

/// Split `text` into chunks of at most `max_words` words, repeating `overlap`
/// words between consecutive chunks.
///
/// # Examples
///
/// ```
/// use testing_basics::chunk_words;
///
/// let chunks = chunk_words("a b c d e", 3, 1).unwrap();
/// assert_eq!(chunks, vec!["a b c", "c d e"]);
/// ```
///
/// Invalid settings are an error, not a panic:
///
/// ```
/// use testing_basics::{chunk_words, ChunkError};
///
/// assert_eq!(chunk_words("a b", 0, 0), Err(ChunkError::ZeroSize));
/// ```
pub fn chunk_words(
    text: &str,
    max_words: usize,
    overlap: usize,
) -> Result<Vec<String>, ChunkError> {
    if max_words == 0 {
        return Err(ChunkError::ZeroSize);
    }
    if overlap >= max_words {
        return Err(ChunkError::OverlapTooLarge { overlap, max_words });
    }

    let words: Vec<&str> = text.split_whitespace().collect();
    let step = max_words - overlap;
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < words.len() {
        let end = (start + max_words).min(words.len());
        chunks.push(words[start..end].join(" "));
        if end == words.len() {
            break;
        }
        start += step;
    }
    Ok(chunks)
}

/// Fraction of a budget used, as a percentage. Panics if `budget` is zero,
/// because a zero budget is a programming error, not bad user input.
pub fn percent_used(used: u64, budget: u64) -> f64 {
    assert!(budget > 0, "budget must be positive");
    used as f64 / budget as f64 * 100.0
}

// Unit tests live next to the code, in a module compiled only for `cargo test`.
// They can test private functions too.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overlap_splits_evenly() {
        let chunks = chunk_words("one two three four", 2, 0).unwrap();
        assert_eq!(chunks, ["one two", "three four"]);
    }

    #[test]
    fn last_chunk_can_be_short() {
        let chunks = chunk_words("a b c d e", 2, 0).unwrap();
        assert_eq!(chunks.last().map(String::as_str), Some("e"));
    }

    #[test]
    fn empty_text_gives_no_chunks() {
        assert_eq!(chunk_words("   ", 5, 1), Ok(vec![]));
    }

    #[test]
    fn overlap_must_be_smaller_than_size() {
        assert_eq!(
            chunk_words("a b c", 2, 2),
            Err(ChunkError::OverlapTooLarge {
                overlap: 2,
                max_words: 2
            })
        );
    }

    // A test can return Result and use `?` instead of unwrap().
    #[test]
    fn every_chunk_respects_the_limit() -> Result<(), ChunkError> {
        let text = "w ".repeat(103);
        for chunk in chunk_words(&text, 10, 3)? {
            assert!(chunk.split_whitespace().count() <= 10, "too long: {chunk}");
        }
        Ok(())
    }

    #[test]
    #[should_panic(expected = "budget must be positive")]
    fn zero_budget_panics() {
        percent_used(5, 0);
    }

    #[test]
    fn percent_used_is_approximately_right() {
        let p = percent_used(1, 3);
        // Never compare floats with ==. Compare with a tolerance.
        assert!((p - 33.333).abs() < 0.001, "got {p}");
    }

    #[test]
    #[ignore = "slow: run with `cargo test -p testing-basics -- --ignored`"]
    fn large_document() {
        let text = "token ".repeat(2_000_000);
        let chunks = chunk_words(&text, 512, 64).unwrap();
        assert_eq!(chunks.len(), 4465);
    }
}
