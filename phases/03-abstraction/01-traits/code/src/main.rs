//! Phase 03 · Lesson 01 — Traits: Python Protocols with teeth
//!
//! Run:  cargo run -p traits
//! Test: cargo test -p traits
//!
//! Scenario: similarity metrics for vector search. Every metric "can score two
//! vectors": that shared ability is a trait. Each metric is its own type.

use std::fmt;

/// The contract: anything that can score two vectors.
/// Python: `class Scorer(Protocol): def score(self, a, b) -> float: ...`
trait Scorer {
    /// Required method: every implementor must provide it.
    fn score(&self, a: &[f32], b: &[f32]) -> f32;

    /// Required: a short name for logs.
    fn name(&self) -> &'static str;

    /// Provided (default) method: implementors get it for free, and may override it.
    fn higher_is_better(&self) -> bool {
        true
    }

    /// Another default method, built on the required ones.
    fn best_match(&self, query: &[f32], candidates: &[Vec<f32>]) -> Option<usize> {
        let scored = candidates.iter().map(|c| self.score(query, c)).enumerate();
        if self.higher_is_better() {
            scored.max_by(|a, b| a.1.total_cmp(&b.1)).map(|(i, _)| i)
        } else {
            scored.min_by(|a, b| a.1.total_cmp(&b.1)).map(|(i, _)| i)
        }
    }
}

struct Dot;
struct Cosine;
struct Euclidean;

impl Scorer for Dot {
    fn score(&self, a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }
    fn name(&self) -> &'static str {
        "dot"
    }
}

impl Scorer for Cosine {
    fn score(&self, a: &[f32], b: &[f32]) -> f32 {
        let dot = Dot.score(a, b);
        let na = Dot.score(a, a).sqrt();
        let nb = Dot.score(b, b).sqrt();
        if na == 0.0 || nb == 0.0 {
            0.0
        } else {
            dot / (na * nb)
        }
    }
    fn name(&self) -> &'static str {
        "cosine"
    }
}

impl Scorer for Euclidean {
    fn score(&self, a: &[f32], b: &[f32]) -> f32 {
        a.iter()
            .zip(b)
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }
    fn name(&self) -> &'static str {
        "euclidean"
    }
    // Override the default: for a distance, lower is better.
    fn higher_is_better(&self) -> bool {
        false
    }
}

// ---- Implementing standard-library traits on your own types ------------------
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Similarity(f32);

// Display: how it prints with {}. Python's __str__.
impl fmt::Display for Similarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0 * 100.0)
    }
}

// From: a conversion. Gives you `Similarity::from(x)` and `x.into()` for free.
impl From<f32> for Similarity {
    fn from(x: f32) -> Self {
        Similarity(x.clamp(-1.0, 1.0))
    }
}

// Default: the "zero" value.
impl Default for Similarity {
    fn default() -> Self {
        Similarity(0.0)
    }
}

// ---- Implementing your trait for a type you didn't write ----------------------
/// Anything that can summarise itself for a log line.
trait Describe {
    fn describe(&self) -> String;
}

impl Describe for [f32] {
    fn describe(&self) -> String {
        format!("vec(dim={}, first={:?})", self.len(), self.first())
    }
}

fn main() {
    let query = [1.0, 0.0, 1.0];
    let docs = vec![
        vec![1.0, 0.0, 0.9],   // almost the same direction
        vec![10.0, 0.0, 10.0], // same direction, much longer
        vec![0.0, 1.0, 0.0],   // orthogonal
    ];

    // The same code works with any Scorer: the trait is the interface.
    let scorers: [&dyn Scorer; 3] = [&Dot, &Cosine, &Euclidean];
    for s in scorers {
        let scores: Vec<String> = docs
            .iter()
            .map(|d| format!("{:>6.3}", s.score(&query, d)))
            .collect();
        println!(
            "{:<9} scores [{}] best = doc {:?}",
            s.name(),
            scores.join(", "),
            s.best_match(&query, &docs)
        );
    }

    let sim: Similarity = Cosine.score(&query, &docs[0]).into();
    println!("Display  : {sim}");
    println!("From     : {}", Similarity::from(3.7));
    println!("Default  : {}", Similarity::default());
    println!("PartialOrd: {}", Similarity(0.2) < Similarity(0.9));
    println!("Describe : {}", query.describe());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_ignores_length_dot_does_not() {
        let a = [1.0, 1.0];
        let b = [5.0, 5.0];
        assert!((Cosine.score(&a, &b) - 1.0).abs() < 1e-6);
        assert_eq!(Dot.score(&a, &b), 10.0);
    }

    #[test]
    fn cosine_of_zero_vector_is_zero_not_nan() {
        assert_eq!(Cosine.score(&[0.0, 0.0], &[1.0, 2.0]), 0.0);
    }

    #[test]
    fn euclidean_prefers_the_closest() {
        let docs = vec![vec![9.0, 9.0], vec![1.0, 1.1]];
        assert_eq!(Euclidean.best_match(&[1.0, 1.0], &docs), Some(1));
    }

    #[test]
    fn best_match_of_nothing_is_none() {
        assert_eq!(Dot.best_match(&[1.0], &[]), None);
    }

    #[test]
    fn from_clamps() {
        assert_eq!(Similarity::from(-7.0), Similarity(-1.0));
    }
}
