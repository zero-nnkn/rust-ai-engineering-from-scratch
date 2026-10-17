# Unit, integration & doc tests

> **Motto:** Tests are built into the language. Use all three kinds.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 00 · Setup & Tooling | 04 | Build | ~1 h | `outputs/test-template.rs` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

In Python, testing means picking a framework (`pytest`), a layout (`tests/` or next to the
code), a fixture style, and plugins for coverage and async. Docstring examples (`doctest`)
exist but most projects don't run them, so they go stale.

Rust builds testing into the compiler and Cargo. There are three kinds of test, each with a
fixed place, and `cargo test` runs all of them. Tests also matter more than usual in this
course: in Part 2, **tests are how you tell a coding agent what "correct" means.**

## Concept

| Kind | Where | Can see | Use for | pytest analogy |
|---|---|---|---|---|
| **unit** | `#[cfg(test)] mod tests` at the bottom of the same file | private items too | small pieces of logic | test file next to the module |
| **integration** | `tests/*.rs` | only the public API | how a user calls your crate | `tests/` folder, importing the package |
| **doc** | ` ``` ` blocks in `///` comments | only the public API | examples that must stay true | `doctest`, but always run |

`#[cfg(test)]` means "compile this only when testing", so test code never ends up in your binary.

**The toolkit:**

| Rust | pytest equivalent |
|---|---|
| `#[test] fn name() { ... }` | `def test_name(): ...` |
| `assert!(cond, "msg {x}")` | `assert cond, f"msg {x}"` |
| `assert_eq!(left, right)` / `assert_ne!` | `assert left == right` (shows both sides on failure) |
| `#[should_panic(expected = "...")]` | `with pytest.raises(...)` for panics |
| test returning `Result<(), E>`, using `?` | test that propagates exceptions |
| `#[ignore = "reason"]` + `-- --ignored` | `@pytest.mark.slow` + `-m slow` |
| `cargo test chunk` | `pytest -k chunk` |
| `cargo test -- --nocapture` | `pytest -s` |

Where the analogy breaks: **errors and panics are different things** (Phase 01, Lesson 05).
Bad *input* should return an `Err`, and you test it with `assert_eq!(f(x), Err(...))`.
`#[should_panic]` is only for *bugs*, such as violating a documented precondition. There are no
fixtures either: a plain helper function that builds test data does the same job.

Tests run **in parallel** by default, on several threads. Tests that share state (a file, an
environment variable) can interfere with each other.

## Explore

The library is a text chunker, the first step of a RAG pipeline. Run all its tests:

```bash
cargo test -p testing-basics
```

Expected summary (one block per kind):

```
     Running unittests src/lib.rs
test tests::large_document ... ignored, slow: run with `cargo test -p testing-basics -- --ignored`
test result: ok. 7 passed; 0 failed; 1 ignored; ...
     Running tests/chunking.rs
test result: ok. 3 passed; 0 failed; 0 ignored; ...
   Doc-tests testing_basics
test result: ok. 2 passed; 0 failed; 0 ignored; ...
```

Then try:

```bash
cargo test -p testing-basics overlap                       # filter by name
cargo test -p testing-basics --release -- --ignored        # the slow test, optimized
cargo test -p testing-basics --doc                         # only doc tests
```

**Break it on purpose.**

1. In `chunk_words`, change `let step = max_words - overlap;` to `let step = max_words;`.
   Run the tests. Read how `assert_eq!` shows both sides. Which tests caught it, and which kinds are they?
2. Undo that. Now change the doc example's expected value to `vec!["a b c", "d e"]` and run
   `cargo test -p testing-basics --doc`. Docs that lie fail the build.

## Practice

1. Add a unit test for text containing newlines and tabs (`"a\tb\nc"`). What does `split_whitespace` do with them?
2. Add an integration test file `tests/properties.rs` that checks, for sizes 1..=10 and every
   valid overlap, that the **first word of each chunk** appears in the original text in order.
   (A loop inside one test is fine.)
3. `percent_used` panics on a zero budget. Write a second version `try_percent_used` that
   returns `Option<f64>` instead, with tests. When is a panic the right choice, and when isn't it?
4. Make one test fail, then run `cargo test -p testing-basics -- --nocapture` with a `println!`
   in it. Why is the output hidden by default?

## Ship It

`outputs/test-template.rs`: a starting skeleton with a unit-test module, a `Result`-returning
test, a `should_panic` test, an ignored slow test, and a float comparison with a tolerance.

## Check Your Understanding

1. Why can't an integration test in `tests/` call a private function?
2. When should a test use `#[should_panic]`, and when `assert_eq!(..., Err(...))`?
3. Why are doc tests more trustworthy than examples in a README?
4. Why is `assert_eq!(0.1 + 0.2, 0.3)` a bad test, and what should you write instead?

## Further Reading

- *The Rust Programming Language*, chapter 11: "Writing Automated Tests"
- *The rustdoc book*: "Documentation tests"
