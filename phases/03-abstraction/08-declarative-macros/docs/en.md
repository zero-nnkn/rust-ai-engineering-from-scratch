# macro_rules! basics

> **Motto:** A macro writes code for you at compile time. Use one when a function can't do the job.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 08 | Learn | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

You've used macros since your first program: `println!`, `vec!`, `format!`, `assert_eq!`. Each one
does something a function can't: take a variable number of arguments, check a format string at
compile time, or generate code. In AI code you'll meet macros constantly in libraries, especially
**derive macros** like `#[derive(Serialize)]` (serde) and `#[derive(Error)]` (thiserror).

Python solves similar problems at runtime with `*args`, decorators, metaclasses and `exec`. Rust
does it at compile time: the macro expands into ordinary Rust, which is then type-checked like
code you wrote yourself.

This lesson teaches you to read and write simple `macro_rules!` macros, and when *not* to.

## Concept

### A macro matches tokens and produces code

```rust
macro_rules! ensure {
    ($cond:expr, $($fmt:tt)+) => {          // pattern: an expression, a comma, then any tokens
        if !$cond {                           // expansion: ordinary Rust
            return Err(format!($($fmt)+));
        }
    };
}

ensure!(max_tokens > 0, "max_tokens must be positive");
// expands to:
// if !(max_tokens > 0) { return Err(format!("max_tokens must be positive")); }
```

A function can't do this: it can't `return` from its *caller*.

### Fragment types

| Fragment | Matches | Example |
|---|---|---|
| `$x:expr` | an expression | `a + 1`, `format!("x")` |
| `$x:ident` | an identifier | `user`, `Search` |
| `$x:ty` | a type | `Vec<u32>` |
| `$x:literal` | a literal | `"search"`, `42` |
| `$x:tt` | one token tree (anything) | used for "pass the rest through" |
| `$x:pat`, `$x:block`, `$x:path` | patterns, blocks, paths | less common |

### Repetition

`$( ... ),*` repeats a pattern zero or more times, separated by commas (`+` for one or more). The
same `$( ... )*` in the expansion repeats the output once per match. `$(,)?` accepts an optional
trailing comma, which is good manners for list-like macros.

```rust
macro_rules! messages {
    ($($role:ident : $content:expr),* $(,)?) => {
        vec![$(Message { role: stringify!($role), content: ($content).to_string() }),*]
    };
}
```

### Three kinds of macro

| Kind | Looks like | You'll |
|---|---|---|
| declarative (`macro_rules!`) | `name!(...)` | read often, write sometimes (this lesson) |
| derive | `#[derive(Serialize, Debug)]` | use constantly (serde, thiserror); write rarely |
| attribute / function-like procedural | `#[tokio::main]`, `sqlx::query!(...)` | use; writing them is advanced (a separate crate) |

### When to use a macro, and when not to

Use one when a function **can't** do it: variable arguments, early return from the caller,
generating items (enums, impls, tests) from one list, or compile-time checks. `tool_enum!` is a good
example: one list of `Variant => "name"` generates the enum, `name()` and `from_name()`, so the three
can never drift apart.

Otherwise prefer a function or generics. Macros are harder to read, harder to debug (errors point
inside the expansion), and invisible to some tooling. In Part 2, an agent writing a macro to avoid a
bit of repetition is often worth questioning.

### Where the analogy breaks

Python decorators and metaclasses run at import time and can do anything. A Rust macro only
rearranges tokens into code, at compile time, and the result is then fully type-checked. So a macro
can't produce a type error at runtime. But it also doesn't understand meaning: `messages![usr: "x"]`
compiles, because the macro only sees an identifier.

## Explore

```bash
cargo run -p declarative-macros
cargo test -p declarative-macros
```

Expected output:

```
messages! :    system: You are terse.
messages! :      user: Name a Rust macro.
messages! : assistant: println!
string_map: 2 entries, keys ["input", "output"]
ensure!   : Ok(())
ensure!   : Err("temperature 3 out of range")
tool_enum : ["search", "calculator", "read_file"]
tool_enum : Some(ReadFile) / None
built-in  : [1, 2, 3] (vec!, format!, assert_eq!, println!)
```

To see what a macro expands to, install `cargo-expand` (`cargo install cargo-expand`; it needs a
nightly toolchain installed alongside) and run `cargo expand -p declarative-macros`. Or ask your tutor
to expand one invocation by hand and check it against the pattern.

**Break it on purpose.**

1. Remove `$(,)?` from the `messages!` pattern. The invocation in `main` ends with a trailing comma.
   Read the error: "unexpected end of macro invocation". Macro errors are about tokens, not types.
2. In `tool_enum!(ToolName { ... })`, change `"read_file"` to `"search"`. Run
   `cargo clippy -p declarative-macros -- -D warnings`. Read "unreachable pattern": the generated
   `match` has two arms for `"search"`. The macro turned a data mistake into a compile error.
3. Change `user:` to `usr:` in the `messages!` call. It compiles. Compare with Phase 01 · Lesson 04's
   `Role` enum: how would you make the macro reject unknown roles? (Hint: expand to `Role::$role`.)

## Practice

1. Make `messages!` produce `Role` enum values instead of strings, so `usr:` fails to compile.
2. Write `assert_close!(a, b)` and `assert_close!(a, b, tol)` (two rules in one macro) for comparing floats.
3. Extend `tool_enum!` to also generate a `description()` method from a third column:
   `Search => "search" : "Search the docs"`.
4. Rewrite `string_map!` as a generic function taking `&[(&str, V)]`. Which one would you rather see
   in a code review, and why?

## Check Your Understanding

1. Name two things a macro can do that a function can't.
2. What do `$x:expr`, `$x:ident` and `$( ... ),*` mean?
3. Why does a macro's output still get fully type-checked?
4. When would you push back on an agent that writes a new macro?

## Further Reading

- *The Rust Programming Language*, chapter 20.5 "Macros"
- *The Little Book of Rust Macros* (online)
