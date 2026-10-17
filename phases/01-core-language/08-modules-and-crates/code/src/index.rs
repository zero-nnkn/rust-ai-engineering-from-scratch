//! A minimal document index. Its ranking logic lives in a child module.

mod ranking; // src/index/ranking.rs, private to `index`

use crate::text; // absolute path from the crate root
use std::collections::HashMap;

/// A search result.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub doc_id: usize,
    pub score: f64,
}

#[derive(Default)]
pub struct Index {
    docs: Vec<String>,
    // pub(crate): visible anywhere inside this crate, invisible to users of the crate.
    pub(crate) term_counts: Vec<HashMap<String, u32>>,
}

impl Index {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, doc: &str) -> usize {
        let mut counts = HashMap::new();
        for w in text::words(doc) {
            *counts.entry(w).or_insert(0) += 1;
        }
        self.term_counts.push(counts);
        self.docs.push(doc.to_string());
        self.docs.len() - 1
    }

    pub fn get(&self, doc_id: usize) -> Option<&str> {
        self.docs.get(doc_id).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<SearchHit> {
        let terms = text::words(query);
        let mut hits: Vec<SearchHit> = (0..self.len())
            .map(|doc_id| SearchHit {
                doc_id,
                score: ranking::score(self, doc_id, &terms),
            })
            .filter(|h| h.score > 0.0)
            .collect();
        hits.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.doc_id.cmp(&b.doc_id)));
        hits.truncate(top_k);
        hits
    }
}
