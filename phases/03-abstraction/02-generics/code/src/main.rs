//! Phase 03 · Lesson 02 — Generics & trait bounds
//!
//! Run:  cargo run -p generics
//! Test: cargo test -p generics
//!
//! Scenario: reusable building blocks for an AI pipeline:
//!   top_k       works for any items and any score type
//!   LruCache    caches anything by any hashable key (here: text → embedding)
//!   Embedder    a trait; embed_all is generic over every embedder

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Keep the `k` best items by score. Generic over the item type `T` and the
/// score type `S`; `S` must be comparable (PartialOrd) and copyable (Copy).
fn top_k<T, S, F>(items: &[T], k: usize, score: F) -> Vec<(&T, S)>
where
    S: PartialOrd + Copy,
    F: Fn(&T) -> S,
{
    let mut scored: Vec<(&T, S)> = items.iter().map(|it| (it, score(it))).collect();
    // partial_cmp returns None for NaN; treat those as equal rather than panicking.
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(k);
    scored
}

/// A tiny least-recently-used cache. Works for any key that can be hashed,
/// compared and cloned, and any value.
struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>, // front = least recently used
    hits: u32,
    misses: u32,
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        LruCache {
            capacity: capacity.max(1),
            map: HashMap::new(),
            order: VecDeque::new(),
            hits: 0,
            misses: 0,
        }
    }

    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            if let Some(k) = self.order.remove(pos) {
                self.order.push_back(k);
            }
        }
    }

    /// Return the cached value, or compute it with `make`, store it and return it.
    fn get_or_insert_with(&mut self, key: K, make: impl FnOnce() -> V) -> &V {
        if self.map.contains_key(&key) {
            self.hits += 1;
            self.touch(&key);
        } else {
            self.misses += 1;
            if self.map.len() == self.capacity {
                if let Some(oldest) = self.order.pop_front() {
                    self.map.remove(&oldest);
                }
            }
            self.order.push_back(key.clone());
            self.map.insert(key.clone(), make());
        }
        &self.map[&key]
    }
}

/// Anything that turns text into a vector.
trait Embedder {
    fn dim(&self) -> usize;
    fn embed(&self, text: &str) -> Vec<f32>;
}

/// A toy embedder: counts characters into `dim` buckets. Deterministic, no model needed.
struct HashingEmbedder {
    dim: usize,
}

impl Embedder for HashingEmbedder {
    fn dim(&self) -> usize {
        self.dim
    }
    fn embed(&self, text: &str) -> Vec<f32> {
        let mut v = vec![0.0; self.dim];
        for c in text.chars().filter(|c| c.is_alphanumeric()) {
            v[(c.to_ascii_lowercase() as usize) % self.dim] += 1.0;
        }
        v
    }
}

/// Static dispatch: the compiler generates a copy of this function for each
/// concrete embedder type it's called with (monomorphization). No runtime cost.
fn embed_all<E: Embedder>(embedder: &E, texts: &[&str]) -> Vec<Vec<f32>> {
    texts.iter().map(|t| embedder.embed(t)).collect()
}

/// The same idea with `impl Trait` in argument position: shorter, same meaning.
fn describe(embedder: &impl Embedder) -> String {
    format!("embedder with dim {}", embedder.dim())
}

fn main() {
    // top_k with different item and score types
    let words = ["rust", "ownership", "ai", "borrowing"];
    println!("top_k by len : {:?}", top_k(&words, 2, |w| w.len()));

    let latencies_ms = [120.5, 88.0, 301.2, 95.4];
    println!("top_k f64    : {:?}", top_k(&latencies_ms, 1, |&x| x));

    // LruCache<&str, Vec<f32>>
    let embedder = HashingEmbedder { dim: 8 };
    println!("{}", describe(&embedder));
    let mut cache: LruCache<&str, Vec<f32>> = LruCache::new(2);
    for text in ["hello", "world", "hello", "rust", "world"] {
        let v = cache.get_or_insert_with(text, || embedder.embed(text));
        println!("cache {text:<6}: sum {}", v.iter().sum::<f32>());
    }
    println!("hits/misses  : {}/{}", cache.hits, cache.misses);

    // The same cache type with completely different K and V
    let mut ids: LruCache<u64, String> = LruCache::new(10);
    let label = ids.get_or_insert_with(42, || "answer".to_string());
    println!("LruCache<u64, String>: {label}");

    let batch = embed_all(&embedder, &["a", "bb"]);
    println!(
        "embed_all    : {} vectors of dim {}",
        batch.len(),
        batch[0].len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_keeps_best_and_handles_short_input() {
        let r = top_k(&[3, 1, 2], 5, |&x| x);
        assert_eq!(r.iter().map(|(x, _)| **x).collect::<Vec<_>>(), [3, 2, 1]);
    }

    #[test]
    fn top_k_survives_nan() {
        let r = top_k(&[1.0, f64::NAN, 3.0], 3, |&x| x);
        assert_eq!(r.len(), 3);
    }

    #[test]
    fn lru_evicts_least_recently_used() {
        let mut c: LruCache<&str, u32> = LruCache::new(2);
        c.get_or_insert_with("a", || 1);
        c.get_or_insert_with("b", || 2);
        c.get_or_insert_with("a", || 99); // hit: "a" becomes most recent
        c.get_or_insert_with("c", || 3); // evicts "b"
        assert!(c.map.contains_key("a"));
        assert!(!c.map.contains_key("b"));
        assert_eq!((c.hits, c.misses), (1, 3));
    }

    #[test]
    fn cached_value_is_not_recomputed() {
        let mut c: LruCache<u8, u8> = LruCache::new(1);
        c.get_or_insert_with(1, || 10);
        assert_eq!(*c.get_or_insert_with(1, || panic!("recomputed")), 10);
    }
}
