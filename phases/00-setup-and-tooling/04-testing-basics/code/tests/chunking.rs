//! Integration tests: each file in `tests/` is compiled as a separate crate that
//! can only use the library's *public* API, exactly like a real user would.

use testing_basics::{chunk_words, ChunkError};

#[test]
fn overlapping_chunks_share_words() {
    let chunks = chunk_words("the quick brown fox jumps over", 4, 2).unwrap();
    assert_eq!(
        chunks,
        ["the quick brown fox", "brown fox jumps over"],
        "neighbouring chunks should share 2 words"
    );
}

#[test]
fn all_words_survive_chunking() {
    let text = "alpha beta gamma delta epsilon zeta eta";
    let chunks = chunk_words(text, 3, 0).unwrap();
    let rejoined = chunks.join(" ");
    assert_eq!(rejoined, text);
}

#[test]
fn zero_size_is_rejected() {
    assert_eq!(chunk_words("anything", 0, 0), Err(ChunkError::ZeroSize));
}
