//! Phase 04 · Lesson 05 — Reviewing agent-written Rust
//!
//! The prompt given to the agent was:
//!   "Write a chunk store for our RAG pipeline: split documents into chunks of
//!    at most `max_tokens` words, skip duplicate chunks, support keyword search,
//!    loading from a TSV index, saving, and a token total. Make it compile,
//!    pass clippy, and add tests."
//!
//! It did. `cargo clippy -- -D warnings` is clean and `cargo test` is green.
//! Your job: review it like a senior engineer and find at least 10 problems
//! BEFORE opening the answer key in ../docs/en.md.
//!
//! Test: cargo test -p reviewing-agent-rust

#![allow(clippy::needless_range_loop)]

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub doc_id: String,
    pub text: String,
    pub tokens: usize,
}

#[derive(Default)]
pub struct ChunkStore {
    chunks: Arc<Mutex<Vec<Chunk>>>,
    seen: Vec<String>,
}

impl ChunkStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Split `text` into chunks of at most `max_tokens` words and store them.
    pub fn add_document(&mut self, doc_id: String, text: String, max_tokens: usize) {
        let words: Vec<String> = text.split_whitespace().map(|w| w.to_string()).collect();
        let mut i = 0;
        while i < words.len() {
            let end = std::cmp::min(i + max_tokens, words.len());
            let piece = words[i..end].join(" ");
            if !self.seen.contains(&piece) {
                self.seen.push(piece.clone());
                self.chunks.lock().unwrap().push(Chunk {
                    doc_id: doc_id.clone(),
                    text: piece,
                    tokens: end - i,
                });
            }
            i = end;
        }
    }

    /// Case-insensitive keyword search.
    pub fn search(&self, query: String) -> Vec<Chunk> {
        let chunks = self.chunks.lock().unwrap().clone();
        let q = query.to_lowercase();
        let mut results = vec![];
        for i in 0..chunks.len() {
            if chunks[i].text.to_lowercase().contains(&q) {
                results.push(chunks[i].clone());
            }
        }
        results
    }

    /// Load one line of a TSV index: `doc_id \t tokens \t text`.
    pub fn load_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split('\t').collect();
        let tokens: usize = parts[1].parse().unwrap();
        self.chunks.lock().unwrap().push(Chunk {
            doc_id: parts[0].to_string(),
            text: parts[2].to_string(),
            tokens,
        });
    }

    pub fn export(&self) -> String {
        let chunks = self.chunks.lock().unwrap();
        let mut out = String::new();
        for c in chunks.iter() {
            out += &format!("{}\t{}\t{}\n", c.doc_id, c.tokens, c.text);
        }
        out
    }

    pub fn save(&self, path: &str) {
        let _ = std::fs::write(path, self.export());
    }

    pub fn token_total(&self) -> u32 {
        self.chunks
            .lock()
            .unwrap()
            .iter()
            .map(|c| c.tokens as u32)
            .sum()
    }

    pub fn first_chunk_text(&self) -> String {
        let chunks = self.chunks.lock().unwrap();
        // SAFETY: chunks is never empty when this is called
        unsafe { chunks.get_unchecked(0).text.clone() }
    }

    pub fn len(&self) -> usize {
        self.chunks.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_and_searches() {
        let mut store = ChunkStore::new();
        store.add_document("a".into(), "rust is fast and safe".into(), 2);
        assert_eq!(store.len(), 3);
        assert_eq!(store.search("FAST".into()).len(), 1);
    }

    #[test]
    fn dedups_chunks() {
        let mut store = ChunkStore::new();
        store.add_document("a".into(), "hello world hello world".into(), 2);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn loads_index_line() {
        let mut store = ChunkStore::new();
        store.load_line("doc\t3\tone two three");
        assert_eq!(store.token_total(), 3);
        assert_eq!(store.first_chunk_text(), "one two three");
    }

    #[test]
    fn save_works() {
        let mut store = ChunkStore::new();
        store.add_document("a".into(), "x y z".into(), 10);
        store.save("/nonexistent-dir/out.tsv");
    }
}
