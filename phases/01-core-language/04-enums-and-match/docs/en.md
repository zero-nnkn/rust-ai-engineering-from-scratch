# Enums & pattern matching

> **Motto:** Make invalid states unrepresentable, then let `match` check you handled every valid one.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 04 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

An LLM streaming API sends different kinds of events: a start event with a model name, text
deltas, a usage report, a stop reason, an error. In Python you'd model each event as a dict
with a `"type"` key, and the handler is a chain of `if event["type"] == "text_delta":`.

Two things go wrong, and both are silent:

- A typo (`"text_detla"`) or a renamed type quietly falls through to the `else` branch.
- The provider adds a new event type, and no part of your code notices.

Python's `Enum` only gives you names, not per-case data. Rust enums carry **different data in
each variant**, and `match` refuses to compile until every variant is handled.

## Concept

### Enums are "one of these", with data

```rust
enum StreamEvent {
    Start { model: String },            // struct-like variant
    TextDelta(String),                  // tuple-like variant
    Usage { input: u32, output: u32 },
    Stop(StopReason),                   // a variant can hold another enum
    Error { status: u16, message: String },
}
```

A value of type `StreamEvent` is **exactly one** of these at a time, carrying only that
variant's fields. Compare a struct, which is "all of these at once". Most data modelling in
Rust combines the two: structs for "and", enums for "or".

| Python | Rust |
|---|---|
| `Enum` with plain members | `enum Role { System, User, Assistant }` |
| dict with `"type"` + different keys per type | `enum` with data per variant |
| `Optional[X]` | `Option<X>`, which is just `enum Option { Some(X), None }` |
| `Literal["system", "user"]` checked by mypy, maybe | `enum`, checked by the compiler, always |

### Pattern matching

| Tool | Use | Example |
|---|---|---|
| `match` | handle every variant | `match e { StreamEvent::TextDelta(t) => ..., ... }` |
| guard | refine a pattern | `Error { status, .. } if *status == 429 => ...` |
| `..` | ignore remaining fields | `Usage { output, .. }` |
| `_` | match anything else | `_ => "fail"` |
| `if let` | handle one pattern, ignore the rest | `if let Some(role) = Role::parse(s) { ... }` |
| `let ... else` | destructure or leave early | `let Some(role) = parse(s) else { return; };` |
| `matches!` | a `bool` from a pattern | `matches!(e, StreamEvent::TextDelta(_))` |

**Be careful with `_`.** A catch-all arm switches off the exhaustiveness check: when a new
variant is added, it silently goes to `_`. In `collect` there's no `_` on purpose, so adding a
variant forces a decision.

### Where the analogy breaks

Python 3.10's `match` looks similar but isn't checked: a missing case does nothing at
runtime. Rust's `match` on an enum is a compile-time guarantee. That's why agent-written Rust
in Part 2 is easier to trust: if a new event type appears, the code that ignores it doesn't build.

## Explore

```bash
cargo run -p enums-and-match
cargo test -p enums-and-match
```

Expected output:

```
model : small-model
text  : Ownership means one owner.
tokens: 17
stop  : finished normally
if let: 5 output tokens
matches!: 2 text deltas
role  : Assistant → "assistant"
error : Some("rate limited: slow down")
stop  : cut off: raise max_tokens
stop  : wants to call tool `search`
```

**Break it on purpose.**

1. Add a variant `Ping` to `StreamEvent`. Run `cargo build -p enums-and-match` and read `E0004`:
   every `match` that must handle it is listed. Handle it in `collect`, then remove it again.
2. Delete the `StreamEvent::Error { status, message } =>` arm (the one *without* the guard).
   Why is the guarded arm alone not enough?
3. Write `Role::Usr` anywhere. Compare the error with what a typo in a Python string does.

## Practice

1. Rewrite `decide` from Lesson 02 to return an enum `Decision { Done, Retry { reason: String }, Fail { reason: String } }`,
   and rewrite `run_with_retries` to match on it instead of comparing strings.
2. Replace `role: String` in Lesson 03's `Message` with `role: Role`. What disappears from
   `trim_to_budget`'s logic, and what can no longer go wrong?
3. Add `StreamEvent::ToolCallDelta { id: String, json_fragment: String }` and extend `Response`
   to collect tool-call arguments per `id`. (A `Vec<(String, String)>` is fine for now.)
4. Write `fn is_terminal(e: &StreamEvent) -> bool` with `matches!` that's true for `Stop` and `Error`.

## Check Your Understanding

1. What's the difference between a struct and an enum, in one sentence each?
2. Why is a `_` arm sometimes a bug waiting to happen?
3. When would you use `if let` instead of `match`? And `let ... else`?
4. `Option<T>` is "just an enum". What does that tell you about how `None` is handled in Rust?

## Further Reading

- *The Rust Programming Language*, chapter 6 "Enums and Pattern Matching" and chapter 19 "Patterns and Matching"
