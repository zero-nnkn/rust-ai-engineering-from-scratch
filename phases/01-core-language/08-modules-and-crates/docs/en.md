# Modules, crates & visibility

> **Motto:** Everything is private until you decide it's part of your API.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 08 | Build | ~1.5 h | `outputs/project-template.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

In a Python package, every module and every name can be imported from anywhere:
`from mylib.index._ranking import _score` works, so someone will write it, and now your
"private" helper is part of your API. Refactoring internals breaks other people's code. The
module tree is also implicit: it's whatever files happen to exist.

Rust makes the module tree **declared** and the API **explicit**. Items are private unless marked
`pub`, and the compiler enforces it, including against code in other crates.

## Concept

### The module tree is declared with `mod`

```
src/lib.rs              crate root: `pub mod index; pub mod text;`
src/text.rs             mod text
src/index.rs            mod index: `mod ranking;`
src/index/ranking.rs    mod index::ranking
src/main.rs             a binary crate that depends on the library
tests/public_api.rs     integration tests: another crate, public API only
```

A file is **not** part of the crate until some parent says `mod name;`. The compiler then looks
for `name.rs` (or `name/mod.rs`, the older style) next to the parent.

### Visibility

| Marker | Visible to | Python analogy |
|---|---|---|
| *(none)* | this module and its children | `_name`, but enforced |
| `pub(super)` | the parent module | — |
| `pub(crate)` | anywhere in this crate, not to other crates | "internal" by convention only |
| `pub` | everyone who can reach the module | public name |

A struct can be `pub` while its fields stay private, so users can hold an `Index` but can only
change it through its methods. In this lesson, `term_counts` is `pub(crate)`: the ranking module
reads it, but the binary (a separate crate) can't.

### Paths and `use`

| Path | Means | Python |
|---|---|---|
| `crate::text::words` | absolute, from this crate's root | `from mypkg.text import words` |
| `super::Index` | the parent module | `from .. import Index` |
| `self::ranking` | this module | `from . import ranking` |
| `modules_and_crates::Index` | another crate (dashes become underscores) | `import modules_and_crates` |

`pub use index::{Index, SearchHit};` in `lib.rs` is a **re-export**: users write
`modules_and_crates::Index` without knowing the file layout. You can move `Index` to another
module later without breaking anyone. This is the job `__init__.py` imports do in Python.

### Crates

A **crate** is the unit the compiler builds: one library or one binary. A package (Lesson 00-02)
can contain one library crate plus several binary crates. `main.rs` and every file in `tests/`
are separate crates that use the library through its public API, so they test what real users
can see.

## Explore

```bash
cargo run -p modules-and-crates
cargo test -p modules-and-crates
```

Expected output:

```
modules-and-crates v0.1.0
words  : ["private", "by", "default"]
query  : "rust visibility"
  3.139  #2 Visibility in Rust: pub, pub(crate), and private by default.
  1.223  #0 Rust modules are declared with mod.
query  : "python packages"
  3.833  #1 Python packages use __init__.py files.
query  : "java"
  (no results)
```

Open the files in this order: `lib.rs` → `text.rs` → `index.rs` → `index/ranking.rs` → `main.rs`.
Follow each `mod`, `use` and `pub`.

**Break it on purpose.**

1. In `main.rs`, uncomment `let _ = index.term_counts;`. Read `E0616`: the field is private to
   the binary because it's `pub(crate)`.
2. Remove `pub` from `pub fn words` in `text.rs`. Even `index.rs`, in the same crate, stops
   compiling: private means "this module and its children", not "this crate". Change it to
   `pub(crate) fn words`. Which file still fails now, and why?
3. Comment out `mod ranking;` in `index.rs`. The file still exists, but the compiler no longer
   knows about it. Read the error at `ranking::score`.

## Practice

1. Move `SearchHit` into its own module `src/index/hit.rs`. Keep `main.rs` and the tests
   unchanged: the re-export in `lib.rs` (and possibly a `pub use` in `index.rs`) should make that
   possible.
2. Add a private `mod stopwords` inside `text` and use it in `words`. Write a unit test in `text`
   that checks a stopword is dropped. Can `index.rs` call the stopword list? Should it?
3. Make `ranking::score` `pub` instead of `pub(super)`. Can `main.rs` call it now? (Hint: the
   *module* `ranking` is still private.) What does that teach you about how visibility combines?
4. Run `cargo doc -p modules-and-crates --open`. Which items appear, and why do some not?

## Ship It

`outputs/project-template.md`: a layout and checklist for a small Rust library + CLI, with the
visibility rules to follow from day one.

## Check Your Understanding

1. Why does a file need a `mod` declaration before it becomes part of the crate?
2. What's the difference between `pub(crate)` and `pub`, and why does it matter for `main.rs`?
3. What problem does `pub use` (re-exporting) solve?
4. Why are integration tests in `tests/` limited to the public API, and why is that useful?

## Further Reading

- *The Rust Programming Language*, chapter 7 "Managing Growing Projects with Packages, Crates, and Modules"
- *The Rust Reference*: "Visibility and Privacy"
