//! Phase 03 · Lesson 05 — Iterators: lazy pipelines like generators
//!
//! Run:  cargo run -p iterators
//! Test: cargo test -p iterators
//!
//! Scenario: a data-prep pipeline over a JSONL-like dataset of training examples:
//! filter bad rows, dedupe, window long texts, batch for an embedding API.
//! Plus a custom iterator, the Rust version of a Python generator.

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
struct Example {
    id: u32,
    text: String,
    label: Option<&'static str>,
}

fn dataset() -> Vec<Example> {
    let raw = [
        (1, "Rust has no garbage collector.", Some("fact")),
        (2, "", Some("fact")),
        (3, "Ownership moves values.", None),
        (4, "Rust has no garbage collector.", Some("fact")),
        (5, "Borrowing lends access without moving.", Some("fact")),
        (6, "Iterators are lazy.", Some("tip")),
    ];
    raw.iter()
        .map(|&(id, text, label)| Example {
            id,
            text: text.to_string(),
            label,
        })
        .collect()
}

/// A custom iterator: overlapping word windows over a text.
/// Python: a generator function with `yield`.
struct WordWindows<'a> {
    words: Vec<&'a str>,
    size: usize,
    step: usize,
    pos: usize,
}

impl<'a> WordWindows<'a> {
    fn new(text: &'a str, size: usize, step: usize) -> Self {
        WordWindows {
            words: text.split_whitespace().collect(),
            size: size.max(1),
            step: step.max(1),
            pos: 0,
        }
    }
}

impl Iterator for WordWindows<'_> {
    type Item = String;

    // Implement one method, get ~70 adapter methods (map, filter, take, ...) for free.
    fn next(&mut self) -> Option<String> {
        if self.pos >= self.words.len() {
            return None;
        }
        let end = (self.pos + self.size).min(self.words.len());
        let window = self.words[self.pos..end].join(" ");
        self.pos = if end == self.words.len() {
            end
        } else {
            self.pos + self.step
        };
        Some(window)
    }
}

/// The whole pipeline is lazy until `collect`: no intermediate Vecs.
fn clean(examples: &[Example]) -> Vec<&Example> {
    let mut seen = HashSet::new();
    examples
        .iter()
        .filter(|e| !e.text.trim().is_empty()) // drop empty rows
        .filter(|e| e.label.is_some()) // drop unlabeled rows
        .filter(|e| seen.insert(e.text.as_str())) // keep first occurrence only
        .collect()
}

fn main() {
    let data = dataset();
    let cleaned = clean(&data);
    let ids: Vec<u32> = cleaned.iter().map(|e| e.id).collect();
    println!("cleaned ids   : {ids:?}");

    // Aggregations: count, sum, max_by_key, any, all, position
    let total_words: usize = cleaned
        .iter()
        .map(|e| e.text.split_whitespace().count())
        .sum();
    let longest = cleaned.iter().max_by_key(|e| e.text.len()).map(|e| e.id);
    println!("total words   : {total_words}");
    println!("longest id    : {longest:?}");
    println!(
        "any tip?      : {}",
        cleaned.iter().any(|e| e.label == Some("tip"))
    );
    println!(
        "all end in '.': {}",
        cleaned.iter().all(|e| e.text.ends_with('.'))
    );

    // Batching for an API that accepts 2 texts per request
    let batches: Vec<Vec<&str>> = cleaned
        .chunks(2)
        .map(|chunk| chunk.iter().map(|e| e.text.as_str()).collect())
        .collect();
    println!(
        "batches       : {} of sizes {:?}",
        batches.len(),
        batches.iter().map(Vec::len).collect::<Vec<_>>()
    );

    // zip, enumerate, rev, skip, step_by
    let labels: Vec<String> = cleaned
        .iter()
        .enumerate()
        .map(|(i, e)| format!("{i}:{}", e.label.unwrap_or("?")))
        .collect();
    println!("enumerate     : {labels:?}");
    let evens: Vec<u32> = (1..=10).rev().step_by(2).collect();
    println!("rev/step_by   : {evens:?}");

    // partition and fold
    let (facts, other): (Vec<&Example>, Vec<&Example>) =
        cleaned.iter().partition(|e| e.label == Some("fact"));
    println!(
        "partition     : {} facts, {} other",
        facts.len(),
        other.len()
    );
    let csv = cleaned.iter().fold(String::new(), |acc, e| {
        if acc.is_empty() {
            e.id.to_string()
        } else {
            format!("{acc},{}", e.id)
        }
    });
    println!("fold          : {csv}");

    // Custom iterator + adapters. Laziness: take(2) stops the windowing early.
    let long = "one two three four five six seven";
    let windows: Vec<String> = WordWindows::new(long, 3, 2).take(2).collect();
    println!("windows take 2: {windows:?}");
    println!("windows count : {}", WordWindows::new(long, 3, 2).count());

    // Collecting into different types: the type annotation picks the result.
    let words: HashSet<&str> = long.split_whitespace().filter(|w| w.len() > 3).collect();
    let mut sorted: Vec<_> = words.into_iter().collect();
    sorted.sort();
    println!("collect set   : {sorted:?}");
    let parsed: Result<Vec<u32>, _> = ["1", "2", "x"].iter().map(|s| s.parse::<u32>()).collect();
    println!(
        "collect Result: {}",
        if parsed.is_err() {
            "Err (stops at the first bad item)"
        } else {
            "Ok"
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_drops_empty_unlabeled_and_duplicates() {
        let d = dataset();
        let ids: Vec<u32> = clean(&d).iter().map(|e| e.id).collect();
        assert_eq!(ids, [1, 5, 6]);
    }

    #[test]
    fn windows_cover_the_end_and_overlap() {
        let w: Vec<String> = WordWindows::new("a b c d e", 2, 2).collect();
        assert_eq!(w, ["a b", "c d", "e"]);
        let w: Vec<String> = WordWindows::new("a b c d", 3, 1).collect();
        assert_eq!(w, ["a b c", "b c d"]);
    }

    #[test]
    fn windows_of_empty_text_is_empty() {
        assert_eq!(WordWindows::new("   ", 3, 1).count(), 0);
    }

    #[test]
    fn iterators_are_lazy() {
        let mut calls = 0;
        let first_even = (1..1_000_000).inspect(|_| calls += 1).find(|x| x % 2 == 0);
        assert_eq!(first_even, Some(2));
        assert_eq!(calls, 2, "only two items were ever produced");
    }
}
