//! Phase 01 · Lesson 09 — Project: token & word statistics CLI
//!
//! The library: argument parsing and statistics. `main.rs` only does I/O.

pub mod args;
pub mod stats;

pub use args::{parse_args, Config, Input, UsageError};
pub use stats::{Stats, TextStats};
