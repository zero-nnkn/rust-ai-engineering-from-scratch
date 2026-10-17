# Newtype, builder, typestate

> **Motto:** Put the rules in the types, so wrong code doesn't compile.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 07 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Three bugs from real AI codebases:

- An API key ends up in a log file because someone wrote `logger.debug(f"request: {req}")`.
- `max_tokens=4000` was meant as tokens, but a helper passed a **character** count.
- A request was sent to the model before a human approved it, because one code path skipped the
  approval step.

In Python each is prevented by discipline: careful logging, naming conventions, an
`if not approved: raise` check you have to remember everywhere. These three patterns move each rule
into the type system, so breaking it is a compile error instead of an incident.

## Concept

### Newtype: a new type around one value

```rust
pub struct ApiKey(String);       // not a String: can't be printed, compared or passed as one by accident
pub struct Tokens(pub u32);      // not a u32: can't be mixed up with characters or bytes
```

A tuple struct with one field costs nothing at runtime: same size, same machine code. But it's a
distinct type, so you control which traits it has. `ApiKey` gets a hand-written `Debug` that hides
the secret and has no `Display` at all. `Tokens` gets `Add` but not `Mul`. The newtype is also the
standard workaround for the orphan rule (Lesson 01): wrap a foreign type to implement a foreign trait on it.

| Python | Rust newtype |
|---|---|
| `NewType("Tokens", int)`: a hint, erased at runtime, freely mixed with `int` | a real type; mixing it with `u32` is a compile error |
| a class wrapping a secret with a custom `__repr__` | same idea, but nothing can bypass it with `vars()` or `__dict__` |

### Builder: many optional settings, validated once

Rust has no keyword arguments and no default parameter values. For a type with many optional fields,
the idiomatic answer is a builder:

```rust
let req = ChatRequest::builder()
    .model("small-model")
    .user("What is a newtype?")
    .temperature(0.3)
    .build()?;           // validates here: missing model, bad temperature → Err
```

Setters take `self` by value and return `Self`, so calls chain. `build()` is the single place where
the rules are checked, so **any `ChatRequest` that exists is valid**. The rest of the code never
needs to re-check it.

### Typestate: the state is part of the type

```rust
Run<Draft>     → .approve() → Run<Approved> → .send() → Run<Sent>
```

`send` is defined only on `Run<Approved>`. Calling it on a `Run<Draft>` isn't a runtime error;
the method doesn't exist for that type. Each transition **consumes** the old state (takes `self`), so
you can't keep using a draft after approving it. `PhantomData<State>` is a zero-sized marker that
carries the state in the type without storing anything.

Use typestate for workflows where skipping a step is costly: approval before sending, validation
before execution, connect before query. For agents, "a tool call must be approved before it runs" is
a natural fit.

### When not to

- Newtypes add boilerplate (you implement or derive every trait you need). Use them for values that
  are easily confused or sensitive, not for every integer.
- Builders are for types with several optional fields. Two required fields only need a `new(a, b)`.
- Typestate gets awkward when states are decided at runtime (loaded from a database, chosen by the
  model). Then an `enum` state with runtime checks is simpler.

## Explore

```bash
cargo run -p design-patterns
cargo test -p design-patterns
```

Expected output:

```
newtype   : ApiKey(***90)
tokens    : Tokens(1024)
builder   : Ok(("small-model", Tokens(1024), 0.3, true))
builder   : Some("temperature 5 is outside 0..=2")
builder   : Some("at least one message is required")
typestate : drafted
typestate : approved by reviewer
typestate : sent 1 message(s) to small-model with a 18-char key
```

The key is printed as `ApiKey(***90)` even with `{:?}`. The only way to read the full secret is
`expose()`, a private method that's easy to search for in review.

**Break it on purpose.**

1. In `main`, uncomment the idea in the last comment: `Run::new(req).send(&key)`. Read `E0599`:
   "no method named `send` found for struct `Run<Draft>`". The compiler names the state.
2. Add `println!("{key}");`. Read `E0277`: `ApiKey` doesn't implement `Display`, on purpose.
3. Call `.max_tokens(Tokens(512))` instead of `.max_tokens(512)`, then the reverse: make the setter
   take `Tokens` and pass `512`. Which design do you prefer for the public API?

## Practice

1. Add a `Rejected` state: `Run<Draft>::reject(self, reason: &str) -> Run<Rejected>`. A rejected run
   can't be sent or approved. Write a test that the happy path still works.
2. Add a `Chars(u32)` newtype and `fn estimate(chars: Chars) -> Tokens`. Try passing a `Tokens`
   where a `Chars` is expected.
3. Make `ChatRequestBuilder::build` collect **all** validation errors, not just the first, and return
   `Result<ChatRequest, Vec<String>>`. Is that better for a human, for an LLM that's filling in the
   request, or both?
4. The first version of `ApiKey`'s `Debug` sliced bytes: `&self.0[self.0.len() - 2..]`. Explain how it
   could panic (Phase 01 · Lesson 07), and why the test `debug_is_safe_with_non_ascii_keys` exists. This
   is the kind of bug to look for in agent-written code.

## Check Your Understanding

1. What does a newtype cost at runtime, and what does it buy you?
2. Why does a builder's `build()` return a `Result`, and what does that guarantee about the built value?
3. How does typestate turn "forgot to approve" into a compile error?
4. When is an `enum` state better than typestate?

## Further Reading

- *The Rust Programming Language*, chapter 18.3 "Implementing an Object-Oriented Design Pattern" (typestate-style blog post example)
- *Rust API Guidelines*: "Type safety" (newtypes, builders)
