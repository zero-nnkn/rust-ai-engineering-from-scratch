//! Phase 01 · Lesson 08 — Modules, crates & visibility
//!
//! Run:  cargo run -p modules-and-crates
//! Test: cargo test -p modules-and-crates
//!
//! The crate root. Each `mod` line pulls in a module from a file:
//!
//!   src/lib.rs              ← crate root (this file)
//!   src/text.rs             ← mod text
//!   src/index.rs            ← mod index
//!   src/index/ranking.rs    ← mod index::ranking (a child of index)
//!   src/main.rs             ← the binary, a separate crate that uses this library
//!
//! Python: a package directory with `__init__.py` and submodules, except the
//! module tree is declared explicitly with `mod`, not discovered from files.

pub mod index;
pub mod text;

// Re-exports: the public API users see, independent of the internal layout.
// Like `from .index import Index` in a package's __init__.py.
pub use index::{Index, SearchHit};

/// The crate's version, from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
