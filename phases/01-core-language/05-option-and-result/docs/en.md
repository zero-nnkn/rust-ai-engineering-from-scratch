# Option & Result: no None surprises, no exceptions

> **Motto:** If a value can be missing or an operation can fail, the type says so.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 05 | Build | ~2 h | `outputs/option-result-cheatsheet.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Two of the most common production errors in Python AI code:

```python
args = json.loads(tool_call.arguments)
top_k = int(args.get("top_k"))        # TypeError if missing: int(None)
results = search(args["query"])        # KeyError if the model forgot "query"
```

Neither shows up in the function signature. `args.get` returns `None` sometimes, and `int()`
raises sometimes. You find out in production, when a model formats its output slightly
differently from the last 10,000 times.

Rust has no `null` and no exceptions. A value that may be missing is an `Option<T>`; an
operation that may fail returns a `Result<T, E>`. You can't use the value inside without
deciding what happens in the other case.

## Concept

### The two types

```rust
enum Option<T>    { Some(T), None }      // maybe a value
enum Result<T, E> { Ok(T), Err(E) }      // a value, or an error explaining why not
```

They're ordinary enums (Lesson 04), so `match` works. But you'll mostly use the `?` operator
and a few combinators.

### `?`: propagate the failure

```rust
fn parse_search(args: &Args) -> Result<SearchCall, ArgError> {
    let query = required(args, "query")?;     // Err → return it now; Ok(v) → v
    ...
}
```

`?` is "let the exception propagate", except it's visible: it only works inside a function
whose return type can carry the error. A reader sees every point where the function can fail.

### Converting between them

| You have | You want | Use |
|---|---|---|
| `Option<T>` | `Result<T, E>` | `.ok_or(err)` / `.ok_or_else(\|\| err)` |
| `Result<T, E>` | `Option<T>` (discard the error) | `.ok()` |
| `Result<T, E1>` | `Result<T, E2>` | `.map_err(\|e\| ...)` |
| `Option<Result<T, E>>` | `Result<Option<T>, E>` | `.transpose()` (optional argument that must be valid if present) |

### Getting the value out

| Method | On `None`/`Err` | Use when |
|---|---|---|
| `match` / `if let` | you decide | the two cases need different logic |
| `?` | return it to the caller | the caller should decide |
| `.unwrap_or(default)` | use `default` | a sensible default exists |
| `.unwrap_or_default()` | `0`, `""`, empty… | the type's default is right |
| `.map(f)` / `.and_then(f)` | stays `None`/`Err` | transform only the success case |
| `.expect("why it can't fail")` | **panic** with your message | it's a bug if this fails (an invariant) |
| `.unwrap()` | **panic** | tests and quick experiments only |

**Rule for this course:** no `unwrap()`/`expect()` on anything that comes from outside the
program: files, network, user input, environment variables, LLM output. Part 2's review
checklist looks for exactly this.

### Where the analogy breaks

- In Python, any function can raise anything. In Rust, a function that returns `u32` **cannot**
  fail (except by panicking, which is for bugs, not errors).
- `Option` is not "nullable". `Option<String>` and `String` are different types, and a
  `String` is never missing.
- Errors are values. You can store them, collect them, match on them, and test them with
  `assert_eq!`, as the tests in this lesson do.

## Explore

The example parses an LLM's tool-call arguments into a validated `SearchCall`.

```bash
cargo run -p option-and-result
cargo test -p option-and-result
```

Expected output:

```
case 0: ok    SearchCall { query: "rust ownership", top_k: 3, min_score: None }
case 1: ok    SearchCall { query: "borrowing", top_k: 5, min_score: Some(0.7) }
case 2: error missing argument `query`
case 3: error `top_k` should be a number, got "three"
case 4: error `top_k` out of range: 500
case 5: error missing argument `query`
best           : Some(0.91)
first > 0.8    : Some(0.91)
map+default    : "91%"
unwrap_or(0.0) : 0
```

Notice case 5: a blank query counts as missing, because `optional` filters out
whitespace-only values. That's validation a Python `args.get("query")` would have let through.

**Break it on purpose.**

1. Remove the `?` after `required(args, "query")`. Read the error: you now have a
   `Result<&str, ArgError>` where a `&str` is expected. The type system won't let you forget.
2. Change `parse_search`'s return type to `SearchCall` (no `Result`). Read the error on the
   first `?` (`E0277`): `?` needs a function that can return an error.
3. Replace `.transpose()?` with `.transpose().unwrap()`. It compiles. Now change case 1's
   `min_score` to `"high"` and run it: the program panics. That's the difference between an
   error the caller can handle and a production crash.

## Practice

1. Add a `language: Option<String>` argument that must be one of `"en"`, `"vi"`, `"fr"` if
   present. Add an `ArgError::InvalidChoice` variant and a test for each outcome.
2. Write `fn parse_all(raw: &[Args]) -> Result<Vec<SearchCall>, ArgError>` using `?` in a loop.
   Then write a second version that collects *all* errors instead of stopping at the first:
   `-> (Vec<SearchCall>, Vec<ArgError>)`. When would you want each?
3. Rewrite `required` using `match` instead of `ok_or`. Which is easier to read?
4. Find every `.unwrap()` in the `main` and `tests` of this file. Are any of them on external
   input? Is it acceptable where it is?

## Ship It

`outputs/option-result-cheatsheet.md`: the conversion and extraction tables, plus the rule
about `unwrap()` on external input.

## Check Your Understanding

1. What does `?` do on an `Err`, and why does it only work in functions returning `Result` (or `Option`)?
2. When is `.expect("...")` acceptable, and when is it a bug?
3. Why is `Option<f64>` better than using `-1.0` as "no minimum score"?
4. What does `.transpose()` solve in `parse_search`?

## Further Reading

- *The Rust Programming Language*, chapter 9 "Error Handling"
- `std::option` and `std::result` module docs: the method tables at the top
