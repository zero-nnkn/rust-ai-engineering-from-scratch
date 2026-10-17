//! Phase 02 · Lesson 05 — Box, Rc, Arc, RefCell, Mutex
//!
//! Run:  cargo run -p smart-pointers
//! Test: cargo test -p smart-pointers
//!
//! Each section solves a problem plain ownership can't:
//!   Box      a recursive type (a filter expression for a vector search)
//!   Rc       several owners of one read-only value (a shared system prompt)
//!   RefCell  mutation through a shared reference (a cache's hit counter)
//!   Arc + Mutex  shared, mutable state across threads (a token budget)

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// ---------- Box: recursive data ----------------------------------------------
// A filter like: lang == "rust" AND (year >= 2020 OR NOT draft)
// An enum can't contain itself directly (it would be infinitely large), but it
// can contain a Box pointing to another one on the heap.
#[derive(Debug)]
enum Filter {
    Eq(&'static str, &'static str),
    AtLeast(&'static str, i64),
    Not(Box<Filter>),
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
}

type Meta = HashMap<&'static str, &'static str>;

fn matches(f: &Filter, meta: &Meta) -> bool {
    match f {
        Filter::Eq(k, v) => meta.get(k) == Some(v),
        Filter::AtLeast(k, n) => meta
            .get(k)
            .and_then(|s| s.parse::<i64>().ok())
            .is_some_and(|x| x >= *n),
        Filter::Not(inner) => !matches(inner, meta),
        Filter::And(a, b) => matches(a, meta) && matches(b, meta),
        Filter::Or(a, b) => matches(a, meta) || matches(b, meta),
    }
}

// ---------- Rc: shared ownership (one thread) ---------------------------------
#[derive(Debug)]
struct Conversation {
    system_prompt: Rc<String>, // many conversations share one prompt, no copies
    turns: Vec<String>,
}

// ---------- RefCell: interior mutability (one thread) -------------------------
struct EmbeddingCache {
    vectors: HashMap<String, Vec<f32>>,
    hits: RefCell<u32>, // lets `get(&self)` update a counter
}

impl EmbeddingCache {
    fn get(&self, text: &str) -> Option<&Vec<f32>> {
        let found = self.vectors.get(text);
        if found.is_some() {
            *self.hits.borrow_mut() += 1; // checked at runtime: panics if already borrowed
        }
        found
    }
}

// ---------- Arc + Mutex: shared mutable state across threads -------------------
/// Spend tokens from one shared budget in several worker threads.
fn spend_in_parallel(budget: u32, workers: u32, each: u32) -> (u32, u32) {
    let remaining = Arc::new(Mutex::new(budget)); // Arc: shared across threads; Mutex: one writer at a time
    let mut handles = Vec::new();
    for _ in 0..workers {
        let remaining = Arc::clone(&remaining); // cheap: increments a counter, no data copied
        handles.push(thread::spawn(move || {
            let mut granted = 0;
            for _ in 0..each {
                // lock() returns a guard; the lock is released when the guard is dropped.
                let Ok(mut left) = remaining.lock() else {
                    return granted; // another thread panicked while holding the lock
                };
                if *left == 0 {
                    break;
                }
                *left -= 1;
                granted += 1;
            }
            granted
        }));
    }
    let granted: u32 = handles.into_iter().filter_map(|h| h.join().ok()).sum();
    let left = remaining.lock().map(|g| *g).unwrap_or(0);
    (granted, left)
}

fn main() {
    // Box
    let filter = Filter::And(
        Box::new(Filter::Eq("lang", "rust")),
        Box::new(Filter::Or(
            Box::new(Filter::AtLeast("year", 2020)),
            Box::new(Filter::Not(Box::new(Filter::Eq("draft", "true")))),
        )),
    );
    let doc_a: Meta = [("lang", "rust"), ("year", "2018"), ("draft", "false")].into();
    let doc_b: Meta = [("lang", "rust"), ("year", "2017"), ("draft", "true")].into();
    println!(
        "Box     : doc_a {} / doc_b {}",
        matches(&filter, &doc_a),
        matches(&filter, &doc_b)
    );

    // Rc
    let prompt = Rc::new(String::from("You are a careful Rust reviewer."));
    let convos: Vec<Conversation> = (0..3)
        .map(|i| Conversation {
            system_prompt: Rc::clone(&prompt),
            turns: vec![format!("question {i}")],
        })
        .collect();
    println!(
        "Rc      : {} conversations, {} owners of one prompt, same allocation: {}",
        convos.len(),
        Rc::strong_count(&prompt),
        Rc::ptr_eq(&prompt, &convos[2].system_prompt)
    );
    let last = &convos[2];
    println!(
        "Rc      : {:?} asks {:?}",
        last.system_prompt, last.turns[0]
    );
    drop(convos);
    println!(
        "Rc      : after drop, owners = {}",
        Rc::strong_count(&prompt)
    );

    // RefCell
    let cache = EmbeddingCache {
        vectors: [("hello".to_string(), vec![0.1, 0.2])].into(),
        hits: RefCell::new(0),
    };
    for text in ["hello", "bye", "hello"] {
        let _ = cache.get(text); // ignoring the vector itself; we only care about the counter here
    }
    println!("RefCell : cache hits = {}", cache.hits.borrow());

    // Arc + Mutex
    let (granted, left) = spend_in_parallel(1_000, 4, 300);
    println!("Arc+Mutex: granted {granted} tokens across 4 threads, {left} left");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_tree_evaluates() {
        let f = Filter::Not(Box::new(Filter::Eq("x", "1")));
        let m: Meta = [("x", "2")].into();
        assert!(matches(&f, &m));
    }

    #[test]
    fn budget_is_never_overspent() {
        let (granted, left) = spend_in_parallel(500, 8, 100);
        assert_eq!(granted, 500);
        assert_eq!(left, 0);
    }

    #[test]
    fn budget_leftover_when_demand_is_low() {
        assert_eq!(spend_in_parallel(1_000, 2, 100), (200, 800));
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn refcell_panics_on_overlapping_mutable_borrows() {
        let c = RefCell::new(0);
        let _a = c.borrow_mut();
        let _b = c.borrow_mut(); // the runtime version of E0499
    }
}
