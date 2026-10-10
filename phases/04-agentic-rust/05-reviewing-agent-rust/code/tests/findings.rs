//! Tests that encode the behaviour the code SHOULD have.
//! They are #[ignore]d so CI stays green; run them after your review:
//!
//!   cargo test -p reviewing-agent-rust -- --ignored
//!
//! Every one of them fails against the agent's implementation.
//! Your fix is done when they all pass (remove the #[ignore] lines then).

use reviewing_agent_rust::{Chunk, ChunkStore};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
#[ignore = "finding: dedup is global, so identical text in a second document loses that document"]
fn duplicate_text_in_another_document_is_still_attributed() {
    let mut store = ChunkStore::new();
    store.add_document("a".into(), "same words".into(), 5);
    store.add_document("b".into(), "same words".into(), 5);
    let hits = store.search("same".into());
    assert!(
        hits.iter().any(|c| c.doc_id == "b"),
        "doc b vanished: {hits:?}"
    );
}

#[test]
#[ignore = "finding: max_tokens = 0 is never validated and loops forever"]
fn zero_max_tokens_terminates() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut store = ChunkStore::new();
        store.add_document("a".into(), "one two".into(), 0);
        tx.send(()).ok();
    });
    assert!(
        rx.recv_timeout(Duration::from_secs(1)).is_ok(),
        "add_document hung with max_tokens = 0"
    );
}

#[test]
#[ignore = "finding: malformed input panics instead of returning an error"]
fn malformed_index_line_does_not_panic() {
    let result = std::panic::catch_unwind(|| {
        let mut store = ChunkStore::new();
        store.load_line("not a valid line");
    });
    assert!(result.is_ok(), "load_line panicked on bad input");
}

#[test]
#[ignore = "finding: `as u32` silently truncates the total"]
fn token_total_does_not_truncate() {
    let big = u32::MAX as usize + 1;
    let mut store = ChunkStore::new();
    store.load_line(&format!("doc\t{big}\ttext"));
    assert_eq!(store.token_total() as usize, big, "total wrapped around");
}

#[test]
#[ignore = "not a bug: search is correct but clones the whole store on every call (a cost finding)"]
fn search_returns_matching_chunks_only() {
    // This one passes, on purpose: not every review comment is a bug.
    // The problem with `search` is cost (it clones every chunk on each call),
    // which a correctness test can't see. Benchmark it instead (Phase 06).
    let mut store = ChunkStore::new();
    store.add_document("a".into(), "alpha beta gamma".into(), 1);
    let hits: Vec<Chunk> = store.search("beta".into());
    assert_eq!(hits.len(), 1);
}
