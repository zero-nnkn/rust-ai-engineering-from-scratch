# Functions, expressions & control flow

> **Motto:** Almost everything is an expression, so almost everything produces a value.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 02 | Build | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Python separates *statements* (do something) from *expressions* (produce a value). So you
write `x = a if cond else b` for the short case, but need a temporary variable and a full
`if/else` block for anything longer. You can't easily get a value out of a loop either, so you
write flags like `found = None` and `break`, then check the flag afterwards.

Rust treats `if`, `match`, `loop` and `{ blocks }` as expressions. Code that needs flags and
temporaries in Python often becomes one `let` in Rust.

## Concept

### Functions

```rust
fn backoff_ms(attempt: u32, base_ms: u64, cap_ms: u64) -> u64 {
    let factor = 1 << attempt;
    (base_ms * factor).min(cap_ms)      // no `;` → this value is returned
}
```

- Parameter types and the return type are **required**. Inside the body, types are inferred.
- The **last expression** (no semicolon) is the return value. Use `return x;` only for early exits.
- No return type means it returns `()`, the "unit" value, which is Rust's `None`-as-return.

**The semicolon rule.** `a + b` is an expression with a value. `a + b;` is a statement whose
value is `()`. An extra `;` on the last line is the most common beginner error: "expected `u64`,
found `()`".

### Control flow as expressions

| Rust | Python equivalent |
|---|---|
| `let x = if c { a } else { b };` | `x = a if c else b` (but blocks can be any length) |
| `let x = match status { 200..=299 => "ok", _ => "err" };` | `match` / `case`, as an expression |
| `let x = loop { ... break value; };` | a `while True` with a flag variable |
| `let x = { let t = f(); t * 2 };` | a temporary and a line of code |
| `for i in 0..10` / `0..=10` | `range(10)` / `range(11)` |
| `while cond { }` | `while cond:` |
| `'outer: for ... { break 'outer; }` | needs a flag or a function in Python |

**`match` must be exhaustive.** Every possible value has to be covered (the `_` arm catches the
rest). This is checked at compile time, and it becomes very useful with enums in Lesson 04.

Where the analogy breaks: `if` conditions must be a `bool`. There's no truthiness: `if v.len()`
and `if some_string` don't compile. Write `if !v.is_empty()`.

## Explore

The example is a retry policy for flaky LLM API calls: exponential backoff with a cap, and a
decision per HTTP status.

```bash
cargo run -p functions-and-control-flow
cargo test -p functions-and-control-flow
```

Expected output:

```
backoff ms    : 250 500 1000 2000 4000 5000
status 200    : done
status 429    : retry (rate limited)
status 503    : retry (server error)
status 404    : fail (client error)
status 302    : fail (unexpected)
retries       : Some(3)
gives up      : None
client error  : None
wait before 4th attempt: 1750 ms
first > 50 at : Some((1, 1))
```

**Break it on purpose.**

1. Add a `;` after `(base_ms * factor).min(cap_ms)` in `backoff_ms`. Read error `E0308` and
   the `help: remove this semicolon` line.
2. In `decide`, delete the `_ => "fail (unexpected)"` arm. Read `E0004`: the compiler tells you
   exactly which values are not covered (`0_u16..=199_u16`, etc.).
3. Change the condition in `run_with_retries` to `if statuses.len() { ... }`. Read the error:
   no truthiness in Rust.

## Practice

1. Add jitter: write `fn backoff_with_jitter(attempt: u32, base_ms: u64, cap_ms: u64, jitter: f64) -> u64`
   where `jitter` is in `0.0..=1.0` and the result is the backoff times `(1.0 - jitter / 2.0)`.
   Test that it never exceeds `cap_ms`.
2. `decide` returns strings, and `run_with_retries` matches on `"done"` and `d.starts_with("retry")`.
   Write down two ways this could silently break. (Lesson 04 fixes this with an enum.)
3. Rewrite the labeled-break search in `main` as a function `fn find_first_over(grid: &[[i32; 3]], limit: i32) -> Option<(usize, usize)>`
   that uses `return` instead of a label. Which version do you prefer?
4. Write `fn classify_latency(ms: u32) -> &'static str` using `match` with ranges:
   `< 100` fast, `100..=499` ok, `500..=1999` slow, anything else timeout-risk. Test the boundaries.

## Check Your Understanding

1. Why does adding a `;` to the last line of a function change its return type?
2. What does `loop { ... break value; }` replace in Python code?
3. Why must `match` cover every case, and how does `_` help?
4. Why doesn't `if v.len() { }` compile, and what should you write instead?

## Further Reading

- *The Rust Programming Language*, chapters 3.3 "Functions" and 3.5 "Control Flow"
- *Rust by Example*: "Flow of Control"
