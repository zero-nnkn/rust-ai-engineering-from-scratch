//! Integration test: uses only the public API, through the re-exports.

use modules_and_crates::{Index, SearchHit};

#[test]
fn best_match_comes_first() {
    let mut index = Index::new();
    index.add("cats and dogs");
    index.add("rust rust rust");
    index.add("rust and python");
    let hits = index.search("rust", 10);
    assert_eq!(hits.first().map(|h: &SearchHit| h.doc_id), Some(1));
    assert_eq!(hits.len(), 2);
}

#[test]
fn top_k_limits_results() {
    let mut index = Index::new();
    for _ in 0..5 {
        index.add("same text");
    }
    assert_eq!(index.search("text", 3).len(), 3);
}
