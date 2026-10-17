# clippy, rustfmt & reading compiler errors

> **Motto:** The compiler is a teacher, not a gatekeeper. Read the whole message.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 00 · Setup & Tooling | 03 | Learn | ~1 h | `outputs/prompt-explain-compiler-error.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Python tells you about most mistakes when the line runs: a `TypeError` in production, an
`AttributeError` in a branch your tests never hit. Rust tells you at compile time, which
means you'll see **a lot** of errors in your first weeks. People who learn Rust quickly have
one habit in common: they read the *whole* error message instead of guessing a fix.

Next to the compiler sit two tools you'll run constantly:

- **rustfmt** (`cargo fmt`) formats code. There's one style, so formatting is never discussed in review.
- **clippy** (`cargo clippy`) is a linter with hundreds of checks. Many of them teach idiomatic Rust.

## Concept

### Anatomy of a compiler error

```
error[E0382]: borrow of moved value: `docs`          ← 1. code + one-line summary
  --> src/main.rs:12:20                              ← 2. where
   |
 9 |     let docs = sample();
   |         ---- move occurs because `docs` has type `Vec<Document>`,   ← 3. labels: the story
   |              which does not implement the `Copy` trait
10 |     consume(docs);
   |             ---- value moved here
11 |
12 |     println!("{}", docs.len());
   |                    ^^^^ value borrowed here after move               ← 4. the failing spot
   |
help: consider cloning the value if the performance cost is acceptable    ← 5. suggested fix
   |
10 |     consume(docs.clone());
   |                 ++++++++
```

Read it in this order:

1. **The summary** says *what rule* was broken.
2. **The labels** (`---`) tell the *story*: where the value was created, where it moved, where it was used.
3. **The caret** (`^^^`) is where it failed, which is often *not* where the mistake is.
4. **`help:`** is a suggestion, not an order. "Clone it" always compiles, but it's often the wrong design.
5. **The code** (`E0382`) has a long explanation: `rustc --explain E0382`.

### Warnings vs lints vs errors

| Source | Example | Fails the build? |
|---|---|---|
| compiler error | type mismatch, use after move | always |
| compiler warning | unused variable, unused import | no (CI turns warnings into errors) |
| clippy lint | `&Vec<T>` parameter, index loop, needless `return` | only with `-D warnings` |

This repo's CI runs `cargo clippy -- -D warnings`, so every warning must be fixed. **Fix the
cause, don't silence the lint.** `#[allow(clippy::...)]` is for rare cases where you can
explain why the lint is wrong, in a comment next to it.

Lint levels can also be set per crate in `Cargo.toml` under `[lints.clippy]`. This lesson's
crate raises `needless_range_loop` from a warning to an error.

### Where the analogy breaks

`ruff` is the closest Python tool, but clippy knows the *types*. It can tell that a `&Vec<f64>`
parameter should be a `&[f64]`, or that a loop index is only used to read one slice. Ruff only
sees text.

## Explore

```bash
cargo run -p clippy-and-fmt
cargo clippy -p clippy-and-fmt --all-targets -- -D warnings     # clean
```

Expected output:

```
average: 0.688
passing (>= 0.6): ["ana", "chi", "dung"]
average of nothing: None
```

**Break it on purpose.** In `src/main.rs`, add the line `let unused = 5;` inside `main`, then
run `cargo clippy -p clippy-and-fmt`. Read the warning and its `help:` line. Remove it again.

## Practice

1. **Fix the messy version.** From `phases/00-setup-and-tooling/03-clippy-and-fmt/code/`, save a
   copy of `src/main.rs`, then copy `exercises/messy.rs` over it. Run `cargo fmt -p clippy-and-fmt`
   (look at what changed), then `cargo clippy -p clippy-and-fmt`. You should get about 9
   findings: three `&Vec` parameters, two needless `return`s, an index loop (an *error* here,
   because of this crate's `[lints]` table), a manual `+=`, a comparison with `true`, and a
   `format!` inside `println!`. Fix all of them without `#[allow]`.
2. **The bug clippy missed.** The messy version prints `average of nothing: NaN`. No lint
   flags it. Change `average` so the empty case is impossible to ignore (hint: look at the
   clean version's return type). This is the lesson in one line: **tools catch style and
   common mistakes; they don't catch wrong behaviour.**
3. **Look at pedantic.** Run `cargo clippy -p clippy-and-fmt -- -W clippy::pedantic` on the clean
   version. Pick one finding, decide whether you agree, and write down why.
4. **Read an explanation.** Run `rustc --explain E0502` and `rustc --explain E0308`. Write one
   sentence for each in your own words in `LEARNING.md`.

## Ship It

`outputs/prompt-explain-compiler-error.md`: a prompt for your AI tutor that explains an
error the way this lesson reads it (summary → story → failing spot → fixes), and doesn't
just hand back corrected code.

## Check Your Understanding

1. Why is the caret (`^^^`) often not where the real mistake is?
2. `help: consider cloning` always makes the error go away. Why is it often still the wrong fix?
3. What's the difference between a compiler warning and a clippy lint, and why does CI treat both as errors?
4. Why is `&[f64]` a better parameter type than `&Vec<f64>`?

## Further Reading

- `rustc --explain <CODE>` for any error code
- The Clippy lint list: <https://rust-lang.github.io/rust-clippy/master/>
