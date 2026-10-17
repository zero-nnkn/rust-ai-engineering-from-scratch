# Closures & function types

> **Motto:** A closure's trait says what it does with what it captured: read it, change it, or consume it.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 04 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

LLM client code is full of callbacks: a function for each streamed token, a retry policy, a hook
that runs when a response completes, middleware that rewrites prompts. Python's lambdas and inner
functions handle this, but with well-known traps: the late-binding loop variable
(`[lambda: i for i in range(3)]` all return 2), `nonlocal` to mutate captured state, and no way to
know whether a callback will be called once, many times, or keeps references alive.

Rust closures capture their environment too, but the compiler works out *how* each closure uses
what it captured, and expresses it as one of three traits. A function's signature then says
exactly what kind of callback it accepts.

## Concept

### Syntax

```rust
let add_one = |x: i32| x + 1;                 // types usually inferred
let longest = |a: &str, b: &str| if a.len() >= b.len() { a.len() } else { b.len() };
let shout = |s: &str| { let t = s.to_uppercase(); t + "!" };   // block body
```

### Three closure traits

| Trait | The closure… | Can be called | Python analogy |
|---|---|---|---|
| `Fn` | only reads what it captured | many times, even concurrently | a pure lambda |
| `FnMut` | mutates what it captured | many times, one call at a time | an inner function using `nonlocal` |
| `FnOnce` | consumes (moves out) what it captured | **once** | a callback that hands off a resource |

Every closure implements the most capable trait it can: a closure that only reads is `Fn` (and
also `FnMut` and `FnOnce`). A function asking for `FnOnce` accepts any closure; one asking for
`Fn` accepts only closures that don't mutate.

**Choose the loosest bound your function needs:** `FnOnce` if you call it once, `FnMut` if you
call it many times but sequentially, `Fn` if you need to share it or call it concurrently.

### Capturing and `move`

By default a closure borrows what it uses (`&` or `&mut`, whichever it needs). `move` makes it
take ownership instead. You need `move` when the closure outlives the current scope: returned
from a function, stored in a struct, or sent to a thread.

```rust
fn make_redactor(secrets: Vec<String>) -> impl Fn(&str) -> String {
    move |text| { /* uses secrets */ }      // `secrets` now lives inside the closure
}
```

`move` doesn't change which trait the closure implements. A `move` closure that only reads its
captured data is still `Fn`.

### Passing and storing closures

| You want | Write |
|---|---|
| accept a closure (static dispatch) | `fn f<F: Fn(&str) -> bool>(p: F)` or `p: impl Fn(&str) -> bool` |
| return a closure | `-> impl Fn(&str) -> String` |
| store different closures in one `Vec` | `Vec<Box<dyn Fn(String) -> String>>` (Lesson 03) |
| a plain function where a closure is expected | works if the signature matches; type is `fn(&str) -> bool` |

### Where the analogy breaks

Python closures capture *variables* (late binding), so they see later changes. Rust closures
capture *values or borrows* when created, and the borrow checker stops you changing a variable
while a closure holds a borrow to it. The `lambda i` trap can't happen. The trade-off: you
sometimes need `move` or a `clone()` before the closure to give it its own copy.

## Explore

```bash
cargo run -p closures
cargo test -p closures
```

Expected output:

```
tokens        : 5
longest token : "environment."
transcript    : ["Rust closures capture their environment."]
retries       : Ok("answer") after attempts [1, 2, 3]
no retry      : Err("400 bad request")
fn pointer    : true
redacted      : key=*** pw=***
middleware    : "A long prompt with a newline and far too"
sort_by       : [("a", 0.9), ("c", 0.7), ("b", 0.4)]
```

Look at `stream`'s signature. `T: FnMut(&str)` tells you the token callback is called many times
and may change state. `C: FnOnce(String)` tells you the completion callback gets ownership of the
finished text and is called at most once.

**Break it on purpose.**

1. Change `T: FnMut(&str)` to `T: Fn(&str)` in `stream`. Read `E0594` twice (for `count` and
   `longest`): a `Fn` closure can't assign to what it captured.
2. In `make_redactor`, remove `move`. Read `E0373`: the closure would borrow `secrets`, which is
   dropped when the function returns. The `help:` line suggests exactly the fix.
3. In `stream`, call `on_complete` twice (clone `full` for the first call). Read `E0382`: a
   `FnOnce` is consumed by its first call.

## Practice

1. Write `fn make_rate_limiter(max_per_window: u32) -> impl FnMut() -> bool` that returns `true`
   for the first `max_per_window` calls and `false` after. Why must it be `FnMut`?
2. Make `with_retries` also take an `on_retry: impl FnMut(u32, &str)` callback for logging, and
   collect the log lines in a test.
3. The Python trap: build `Vec<Box<dyn Fn() -> usize>>` from `for i in 0..3 { v.push(Box::new(move || i)); }`.
   What does each closure return? Explain why Rust doesn't have Python's late-binding problem.
4. Rewrite `middleware()` to take a config `&[&str]` like Lesson 03's pipeline, building the
   `Box<dyn Fn>` list from names. Compare with the trait-object version: when is a closure enough,
   and when do you want a named type implementing a trait?

## Check Your Understanding

1. What's the difference between `Fn`, `FnMut` and `FnOnce`? Which is the loosest bound to accept?
2. When do you need `move`, and does it change which trait the closure implements?
3. Why does returning a closure use `impl Fn`, but storing several use `Box<dyn Fn>`?
4. Why can't Python's late-binding lambda bug happen in Rust?

## Further Reading

- *The Rust Programming Language*, chapter 13.1 "Closures: Anonymous Functions that Capture Their Environment"
- `std::ops::Fn`, `FnMut`, `FnOnce` docs
