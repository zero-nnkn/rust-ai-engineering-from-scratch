//! Phase 02 · Lesson 07 — Stack vs heap & the real cost of clone()
//!
//! Run:  cargo run --release -p stack-heap-and-cost     (always measure release builds)
//! Test: cargo test -p stack-heap-and-cost

use std::borrow::Cow;
use std::hint::black_box;
use std::mem::size_of;
use std::time::Instant;

/// Only allocate a new string when the text actually needs changing.
/// Cow = "clone on write": either a borrowed &str or an owned String.
fn normalize_quotes(text: &str) -> Cow<'_, str> {
    if text.contains(['“', '”']) {
        Cow::Owned(text.replace(['“', '”'], "\""))
    } else {
        Cow::Borrowed(text) // no allocation for the common case
    }
}

/// Capacity values seen while pushing `n` items one by one.
fn growth_steps(n: usize) -> Vec<usize> {
    let mut v: Vec<f32> = Vec::new();
    let mut steps = vec![v.capacity()];
    for i in 0..n {
        v.push(i as f32);
        if v.capacity() != *steps.last().unwrap_or(&0) {
            steps.push(v.capacity());
        }
    }
    steps
}

fn main() {
    println!("--- sizes on the stack (bytes) ---");
    println!("u8                 {:>5}", size_of::<u8>());
    println!("f32                {:>5}", size_of::<f32>());
    println!(
        "u64 / usize        {:>5} / {}",
        size_of::<u64>(),
        size_of::<usize>()
    );
    println!("&str  (ptr+len)    {:>5}", size_of::<&str>());
    println!("&[f32] (ptr+len)   {:>5}", size_of::<&[f32]>());
    println!("String (ptr+len+cap){:>4}", size_of::<String>());
    println!("Vec<f32>           {:>5}", size_of::<Vec<f32>>());
    println!("Box<[f32; 1024]>   {:>5}", size_of::<Box<[f32; 1024]>>());
    println!("[f32; 1024]        {:>5}", size_of::<[f32; 1024]>());
    println!(
        "Option<Box<u8>>    {:>5}  (None uses the null pointer: free)",
        size_of::<Option<Box<u8>>>()
    );
    println!(
        "Option<u64>        {:>5}  (needs an extra tag)",
        size_of::<Option<u64>>()
    );

    println!("--- Vec growth: capacity while pushing 100 f32s ---");
    println!("{:?}", growth_steps(100));

    println!("--- move vs clone of 200,000 strings (timings vary by machine) ---");
    let docs: Vec<String> = (0..200_000)
        .map(|i| format!("document number {i}"))
        .collect();

    let t = Instant::now();
    let copied = black_box(docs.clone()); // deep copy: 200,000 allocations
    let clone_time = t.elapsed();

    let t = Instant::now();
    let moved = black_box(docs); // move: copies 24 bytes (ptr, len, cap)
    let move_time = t.elapsed();

    println!("clone  {:>10.3?}", clone_time);
    println!("move   {:>10.3?}", move_time);
    println!("same data: {}", moved == copied);

    println!("--- Cow: allocate only when needed ---");
    for text in ["plain text", "“smart” quotes"] {
        let out = normalize_quotes(text);
        let kind = match out {
            Cow::Borrowed(_) => "borrowed (no allocation)",
            Cow::Owned(_) => "owned (allocated)",
        };
        println!("{text:?} → {out:?}: {kind}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_are_small() {
        // A String or Vec is three machine words, no matter how much data it owns.
        assert_eq!(size_of::<String>(), 3 * size_of::<usize>());
        assert_eq!(size_of::<Vec<u8>>(), 3 * size_of::<usize>());
        assert_eq!(size_of::<&str>(), 2 * size_of::<usize>());
    }

    #[test]
    fn null_pointer_optimization() {
        assert_eq!(size_of::<Option<Box<u8>>>(), size_of::<Box<u8>>());
        assert_eq!(size_of::<Option<&str>>(), size_of::<&str>());
    }

    #[test]
    fn with_capacity_avoids_reallocation() {
        let mut v: Vec<u32> = Vec::with_capacity(1_000);
        let before = v.as_ptr();
        for i in 0..1_000 {
            v.push(i);
        }
        assert_eq!(v.as_ptr(), before, "the buffer never moved");
    }

    #[test]
    fn growth_is_geometric() {
        let steps = growth_steps(1_000);
        // Few reallocations for many pushes: each step at least doubles.
        assert!(steps.len() < 12, "{steps:?}");
        for w in steps.windows(2).skip(1) {
            assert!(w[1] >= 2 * w[0], "{steps:?}");
        }
    }

    #[test]
    fn cow_borrows_when_nothing_changes() {
        assert!(matches!(normalize_quotes("abc"), Cow::Borrowed("abc")));
        assert_eq!(normalize_quotes("“a”"), "\"a\"");
    }
}
