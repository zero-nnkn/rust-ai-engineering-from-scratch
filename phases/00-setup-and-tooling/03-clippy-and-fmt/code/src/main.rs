//! Phase 00 · Lesson 03 — clippy, rustfmt & reading compiler errors
//!
//! Run:  cargo run -p clippy-and-fmt
//! Lint: cargo clippy -p clippy-and-fmt --all-targets -- -D warnings
//!
//! This file is the *clean* version. `exercises/messy.rs` is the same program
//! the way it often looks on day one. Practice 1 has you fix it with the tools.

/// Average of a list of scores, or `None` for an empty list.
fn average(scores: &[f64]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    Some(scores.iter().sum::<f64>() / scores.len() as f64)
}

/// Names of everyone at or above `threshold`.
fn passing<'a>(names: &[&'a str], scores: &[f64], threshold: f64) -> Vec<&'a str> {
    names
        .iter()
        .zip(scores)
        .filter(|(_, &s)| s >= threshold)
        .map(|(&n, _)| n)
        .collect()
}

fn main() {
    let names = ["ana", "binh", "chi", "dung"];
    let scores = [0.91, 0.42, 0.77, 0.65];

    match average(&scores) {
        Some(avg) => println!("average: {avg:.3}"),
        None => println!("no scores"),
    }
    println!("passing (>= 0.6): {:?}", passing(&names, &scores, 0.6));
    println!("average of nothing: {:?}", average(&[]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_of_empty_is_none() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn average_of_values() {
        assert_eq!(average(&[1.0, 2.0, 3.0]), Some(2.0));
    }

    #[test]
    fn passing_keeps_names_aligned_with_scores() {
        assert_eq!(passing(&["a", "b", "c"], &[0.9, 0.1, 0.6], 0.6), ["a", "c"]);
    }
}
