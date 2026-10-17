//! Phase 01 · Lesson 06 — Vec, HashMap, HashSet, VecDeque
//!
//! Run:  cargo run -p collections
//! Test: cargo test -p collections
//!
//! Scenario: a tiny retrieval index. Documents in a Vec, an inverted index in a
//! HashMap, stopwords in a HashSet, and a sliding window of recent queries in a
//! VecDeque (an agent's short-term memory).

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

struct Index {
    docs: Vec<String>,                     // list
    postings: HashMap<String, Vec<usize>>, // dict[str, list[int]]
    stopwords: HashSet<&'static str>,      // set
    recent_queries: VecDeque<String>,      // collections.deque(maxlen=3)
    max_recent: usize,
}

impl Index {
    fn new(max_recent: usize) -> Self {
        Index {
            docs: Vec::new(),
            postings: HashMap::new(),
            stopwords: ["a", "an", "the", "is", "of", "and"].into_iter().collect(),
            recent_queries: VecDeque::with_capacity(max_recent),
            max_recent,
        }
    }

    fn tokens(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|w| {
                w.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
            .filter(|w| !w.is_empty() && !self.stopwords.contains(w.as_str()))
            .collect()
    }

    fn add(&mut self, text: &str) -> usize {
        let id = self.docs.len();
        // A HashSet removes duplicate words within one document.
        let unique: HashSet<String> = self.tokens(text).into_iter().collect();
        for word in unique {
            // entry API: get the list for this word, or insert an empty one. One lookup.
            self.postings.entry(word).or_default().push(id);
        }
        self.docs.push(text.to_string());
        id
    }

    /// Rank documents by how many query words they contain.
    fn search(&mut self, query: &str) -> Vec<(usize, usize)> {
        self.remember(query);
        let mut scores: HashMap<usize, usize> = HashMap::new();
        for word in self.tokens(query) {
            if let Some(ids) = self.postings.get(&word) {
                for &id in ids {
                    *scores.entry(id).or_insert(0) += 1;
                }
            }
        }
        let mut ranked: Vec<(usize, usize)> = scores.into_iter().collect();
        // HashMap order is random; sort for a stable result (score desc, then id asc).
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        ranked
    }

    fn remember(&mut self, query: &str) {
        if self.recent_queries.len() == self.max_recent {
            self.recent_queries.pop_front(); // O(1) at both ends, unlike Vec::remove(0)
        }
        self.recent_queries.push_back(query.to_string());
    }
}

fn main() {
    let mut index = Index::new(2);
    for text in [
        "Rust is a systems language",
        "The borrow checker enforces ownership",
        "Ownership and borrowing make Rust memory safe",
        "Python is great for AI orchestration",
    ] {
        index.add(text);
    }

    // Vec: indexing, slicing, get (safe indexing), len
    println!("docs          : {}", index.docs.len());
    println!("docs[0]       : {}", index.docs[0]);
    println!("get(10)       : {:?}", index.docs.get(10)); // None instead of a panic
    println!("first two     : {:?}", &index.docs[..2]);

    for query in ["rust ownership", "AI", "the"] {
        let hits = index.search(query);
        println!("search {query:?}: {hits:?}");
    }
    println!("recent        : {:?}", index.recent_queries);

    // HashMap lookups
    println!("postings[rust]: {:?}", index.postings.get("rust"));
    println!("has 'gc'?     : {}", index.postings.contains_key("gc"));

    // BTreeMap when you need sorted keys (dict order in Python is insertion order;
    // HashMap order is unspecified, BTreeMap order is sorted).
    let vocab: BTreeMap<&str, usize> = index
        .postings
        .iter()
        .map(|(w, ids)| (w.as_str(), ids.len()))
        .collect();
    let first_five: Vec<_> = vocab.iter().take(5).collect();
    println!("vocab (sorted): {first_five:?}");

    // Set operations
    let a: HashSet<String> = index.tokens(&index.docs[1]).into_iter().collect();
    let b: HashSet<String> = index.tokens(&index.docs[2]).into_iter().collect();
    let mut shared: Vec<&String> = a.intersection(&b).collect();
    shared.sort();
    println!("shared words  : {shared:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Index {
        let mut i = Index::new(2);
        i.add("rust rust ownership");
        i.add("ownership is the key");
        i
    }

    #[test]
    fn duplicate_words_in_one_doc_count_once() {
        assert_eq!(sample().postings["rust"], vec![0]);
    }

    #[test]
    fn stopwords_are_not_indexed() {
        let i = sample();
        assert!(!i.postings.contains_key("the"));
        assert!(!i.postings.contains_key("is"));
    }

    #[test]
    fn ranking_is_deterministic() {
        let mut i = sample();
        assert_eq!(i.search("ownership rust"), vec![(0, 2), (1, 1)]);
    }

    #[test]
    fn recent_queries_keep_only_the_last_n() {
        let mut i = sample();
        for q in ["a1", "a2", "a3"] {
            i.search(q);
        }
        assert_eq!(i.recent_queries, ["a2", "a3"]);
    }
}
