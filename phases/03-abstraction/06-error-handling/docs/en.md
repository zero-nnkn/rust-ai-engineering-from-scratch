# Error handling with thiserror & anyhow

> **Motto:** Libraries return errors callers can match on. Applications add context and report the chain.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 06 | Build | ~2.5 h | `outputs/error-template.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Phase 01 · Lesson 05 introduced `Result` and `?` with a small error enum. Real programs have
errors from many sources (file I/O, number parsing, HTTP, JSON, your own validation) and many
layers (a parser inside a config loader inside an agent startup). Two needs pull against each other:

- **A caller wants to react** to specific failures: retry a rate limit, clamp an out-of-range value,
  prompt for a missing key. It needs a type it can `match`.
- **A human wants to understand** what went wrong in a log: "could not start the agent, because the
  config file couldn't be read, because it doesn't exist". That needs context and the chain of causes.

Python's exceptions do the second (tracebacks, `raise ... from e`) but make the first fragile:
which exceptions can a function raise? Nobody knows without reading every function it calls.

## Concept

### The pieces in `std`

| Piece | Role | Python |
|---|---|---|
| an `enum` of failure cases | what a caller can match on | a family of exception classes |
| `impl Display` | the human message | `str(e)` |
| `impl std::error::Error` + `source()` | links to the underlying cause | `raise X from e` / `__cause__` |
| `.map_err(\|e\| ...)` / `From` impls | convert one error into another at a boundary | `except A as e: raise B from e` |
| `Box<dyn Error>` | "any error": convenient for applications and `main` | `except Exception` |

**`Display` shouldn't repeat the source's message.** Each layer says what *it* was doing; the cause
is reachable through `source()`. Printing the whole chain is the reporter's job (`report()` in the
example), which gives you:

```
could not start the agent
  caused by: cannot read config file /definitely/missing/agent.conf
  caused by: No such file or directory (os error 2)
```

### Libraries vs applications

| | Library (a crate others call) | Application (a binary, `main`, a service) |
|---|---|---|
| Error type | a specific `enum`, one variant per case a caller might handle | one catch-all type, any error |
| Goal | let callers `match` and recover | add context, log the chain, exit cleanly |
| By hand (std) | `enum` + `Display` + `Error` (this lesson's `ConfigError`) | `Box<dyn Error>` + a `context()` helper |
| Crate | **`thiserror`**: derives `Display` and `Error` from attributes | **`anyhow`**: `anyhow::Result<T>`, `.context("...")`, chain printing |

### The same code with the crates

This repo builds offline, so the example implements everything by hand. In a real project you'd
write the library error with `thiserror`, and the application with `anyhow`
(check each crate's docs for the current version before adding them):

```rust
// Library: cargo add thiserror
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("cannot read config file {}", path.display())]
    Io { path: PathBuf, #[source] source: std::io::Error },

    #[error("line {line}: expected `key = value`, got {text:?}")]
    Syntax { line: usize, text: String },

    #[error("`{key}` is not a number")]
    BadNumber { key: &'static str, #[source] source: ParseFloatError },

    #[error("`{key}` = {value} is outside {min}..={max}")]
    OutOfRange { key: &'static str, value: f64, min: f64, max: f64 },

    #[error("missing required key `{0}`")]
    Missing(&'static str),
}
```

```rust
// Application: cargo add anyhow
use anyhow::{Context, Result};

fn start_agent(path: &Path) -> Result<String> {
    let config = load_config(path).context("could not start the agent")?;
    Ok(format!("agent ready: {}", config.model))
}

fn main() -> Result<()> {          // on error, prints the message and every "Caused by:"
    println!("{}", start_agent(Path::new("agent.conf"))?);
    Ok(())
}
```

Compare these with `src/main.rs`: `thiserror` generates the `Display` and `Error` impls you wrote
by hand, and `anyhow` is the `Context` type plus `report()`.

### Rules of thumb

1. Make error variants for cases a caller could handle differently. Don't make one per line of code.
2. Keep the cause (`source`) instead of turning it into a string: you can still inspect it later.
3. Add context where you know *what you were trying to do* ("loading config from X").
4. `unwrap()`/`expect()` only for bugs, never for input from outside (Phase 01 · Lesson 05).
5. `panic!` is for broken invariants. An unreadable file isn't one.

## Explore

```bash
cargo run -p error-handling
cargo test -p error-handling
```

Expected output:

```
ok       : Ok(ModelConfig { model: "small-model", temperature: 0.2, max_tokens: 1024 })
error    : line 2: expected `key = value`, got "temperature 0.2"
error    : `temperature` is not a number
  caused by: invalid float literal
error    : `temperature` = 3.5 is outside 0..=2
error    : missing required key `model`
handled  : clamping `temperature` to 2
app error: could not start the agent
  caused by: cannot read config file /definitely/missing/agent.conf
  caused by: No such file or directory (os error 2)
```

The "handled" line is a caller matching one variant (`OutOfRange`) and recovering. The last three
lines are the application-level chain: three layers, each adding what it was doing.

**Break it on purpose.**

1. In `number`, replace `.map_err(|source| ConfigError::BadNumber { key, source })?` with just `?`.
   Read the error (`E0271`): `?` has no way to turn `parse`'s `ParseFloatError` into a `ConfigError`. Then fix it a
   second way: `impl From<ParseFloatError> for ConfigError`. What information do you lose with
   `From`? (Hint: which key failed?)
2. Delete `impl Error for ConfigError`. Which line fails, and why does `.context(...)` need it?
3. Change `ConfigError::Io`'s `Display` to include the source:
   `"cannot read config file {}: {source}"`. Run it and look at the duplicated message in the chain.

## Practice

1. Add a `ConfigError::UnknownKey { line: usize, key: String }` that rejects unexpected keys (typos
   like `temprature`). Should it be an error or a warning? Implement your choice with a test.
2. Write a `fn load_or_default(path: &Path) -> Result<ModelConfig, ConfigError>` that returns a default
   config **only** when the file doesn't exist (`std::io::ErrorKind::NotFound`), and still fails on
   every other error. You'll need to match on the `source`.
3. Write `fn parse_all(texts: &[&str]) -> Result<Vec<ModelConfig>, Vec<(usize, ConfigError)>>` that
   reports every bad config, not just the first.
4. Rewrite `ConfigError` with `thiserror`-style attributes in a comment above the hand-written impls,
   so the mapping is explicit. Use `outputs/error-template.md` as a guide.

## Ship It

`outputs/error-template.md`: copy-paste templates for a library error enum (std-only and `thiserror`)
and an application entry point (std-only and `anyhow`), plus the rules of thumb.

## Check Your Understanding

1. Why should a library return a specific error enum instead of `Box<dyn Error>`?
2. What's `source()` for, and why shouldn't `Display` repeat the source's message?
3. What does `.context("...")` add that `?` alone doesn't?
4. When is converting an error with `From` better than `map_err`, and when is it worse?

## Further Reading

- *The Rust Programming Language*, chapter 9 "Error Handling"
- `std::error::Error` docs
- The `thiserror` and `anyhow` crate docs (check the current versions)
