//! Phase 00 · Lesson 02 — the binary half of the package.
//!
//! Run:  cargo run -p cargo-workflow -- "Your Title Here"
//! Test: cargo test -p cargo-workflow
//!
//! Everything after `--` is passed to the program, not to cargo.

use cargo_workflow::{slugify, word_count};
use std::env;

fn main() {
    // env::args() is sys.argv: the first item is the program path, so skip it.
    let args: Vec<String> = env::args().skip(1).collect();
    let title = if args.is_empty() {
        String::from("Rust for AI Engineers: From Scratch")
    } else {
        args.join(" ")
    };

    println!("title: {title}");
    println!("words: {}", word_count(&title));
    println!("slug:  {}", slugify(&title));

    // Cargo sets these at compile time from Cargo.toml.
    println!(
        "built by {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
