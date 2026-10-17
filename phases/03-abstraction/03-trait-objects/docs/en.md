# Trait objects & dynamic dispatch (dyn Tool)

> **Motto:** Generics when the type is known at compile time; `dyn Trait` when it's decided at runtime.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 03 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

An agent has tools: a calculator, a search client, a file reader, an MCP proxy. They're
different types with different fields, but the agent loop wants one list of them, chosen at
runtime from config or from what the model asks for. The same goes for a preprocessing pipeline
built from a config file, or a set of plugins.

A `Vec<T>` holds one type. Generics (Lesson 02) pick the type at compile time. For "a list of
different types that share a trait, decided at runtime", Rust has **trait objects**: `dyn Trait`.
In Python every call works this way: method lookup happens at runtime, always. In Rust you choose
it explicitly, where you need it.

## Concept

### `dyn Trait` behind a pointer

```rust
let tools: Vec<Box<dyn Tool>> = vec![Box::new(Calculator), Box::new(Lookup { .. })];
for t in &tools {
    t.run("...");      // which `run`? looked up at runtime through the vtable
}
```

A `dyn Tool` has no fixed size (a `Calculator` and a `Lookup` are different sizes), so it always
lives behind a pointer: `Box<dyn Tool>` (owned), `&dyn Tool` (borrowed), `Arc<dyn Tool>` (shared).
That pointer is "fat": two words, one to the data and one to a **vtable**, a table of the
concrete type's method addresses.

```
Box<dyn Tool> ┌──────────┐      ┌───────────────────┐
              │ data ptr ├────► │ Lookup { facts }  │
              │ vtable  ─┼──┐   └───────────────────┘
              └──────────┘  └─► [ name, description, run, drop, size ]  (for Lookup)
```

### Generics vs trait objects

| | Generics `fn f<T: Tool>(t: &T)` | Trait objects `fn f(t: &dyn Tool)` |
|---|---|---|
| Type chosen | at compile time | at runtime |
| Mixed types in one `Vec` | no | yes |
| Call cost | direct call, can be inlined | one indirect call through the vtable |
| Code size | one copy per type (monomorphization) | one copy |
| Typical use | algorithms, containers, hot loops | plugins, tool registries, pipelines from config, middleware |

The pattern in `Registry::register` combines both: **generic on the way in**
(`tool: impl Tool + 'static`), **dynamic once stored** (`Box<dyn Tool>`). Callers pass concrete
types; the registry doesn't need to know them.

`'static` in `impl Tool + 'static` means the tool must not borrow anything short-lived, because
the registry keeps it indefinitely.

### Dyn compatibility (object safety)

Not every trait can be used as `dyn Trait`. A trait object only knows a vtable, so methods must
be callable without knowing the concrete type. Two common blockers:

- **Generic methods** (`fn run_all<I: IntoIterator>(...)`): a vtable can't hold infinitely many versions.
- **Returning `Self`** (`fn duplicate(&self) -> Self`): the caller doesn't know `Self`'s size.

Add `where Self: Sized` to such a method to exclude it from the vtable and keep the trait usable as `dyn`.

### Where the analogy breaks

In Python, `tools: list[Tool]` holding different classes is the default and costs nothing extra
to write. In Rust you opt in with `Box<dyn Tool>`, pay a small runtime cost, and give up generic
methods on that trait. In exchange, everything else stays statically dispatched and checked.

## Explore

```bash
cargo run -p trait-objects
cargo test -p trait-objects
```

Expected output:

```
manifest:
- calculator: Evaluate `a OP b` where OP is + - * /
- facts: Look up a fact by key
call calculator("6 * 7") → Ok("42")
call facts("rust_release") → Ok("2015")
call calculator("1 / 0") → Err("division by zero")
call search("anything") → Err("unknown tool \"search\"")
pipeline → "hello from t"
bad config → Some("unknown transform \"shout\"")
size of &Calculator = 8 bytes, &dyn Tool = 16 bytes (data ptr + vtable ptr)
```

The last line shows the fat pointer: same data pointer, plus the vtable pointer.

**Break it on purpose.**

1. Change `tools: Vec<Box<dyn Tool>>` to `tools: Vec<dyn Tool>`. Read `E0277`: the size of
   `dyn Tool` isn't known at compile time.
2. Add a generic method to the trait:
   `fn run_all<I: IntoIterator<Item = String>>(&self, inputs: I) -> Vec<String> { inputs.into_iter().collect() }`.
   Read `E0038`: the trait is no longer dyn compatible. Then add `where Self: Sized` to that
   method and see it compile again.
3. In `register`, remove `+ 'static`. Read the error and its suggestion. Why does a stored tool
   need it, while a function that only *uses* a tool doesn't?

## Practice

1. Add a `ReadFile` tool that reads a file inside one allowed directory and refuses paths with
   `..`. Register it and test both outcomes.
2. Rewrite the `truncate:` parsing in `pipeline_from_config` with `strip_prefix` instead of slicing
   after `starts_with`. Which version would you rather review in an agent's diff?
3. Add a `fn validate(&self, input: &str) -> Result<(), String> { Ok(()) }` default method to
   `Tool` and call it in `Registry::call` before `run`. Override it in `Calculator`. Notice you
   changed every tool's behaviour without touching most of them.
4. Write a generic version of the registry for one tool type, `struct One<T: Tool> { tool: T }`.
   When would you prefer it to `Registry`?

## Check Your Understanding

1. Why must `dyn Trait` always be behind a pointer?
2. What is a vtable, and what does a call through `&dyn Tool` cost compared to a generic call?
3. Why is "generic on the way in, `dyn` once stored" a useful pattern?
4. What makes a trait not dyn compatible, and how does `where Self: Sized` help?

## Further Reading

- *The Rust Programming Language*, chapter 18.2 "Using Trait Objects to Abstract over Shared Behavior"
- *The Rust Reference*: "Trait objects" and "Dyn compatibility"
