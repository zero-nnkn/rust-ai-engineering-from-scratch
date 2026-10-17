# Structs & methods (classes without inheritance)

> **Motto:** Data in structs, behaviour in `impl` blocks, and the receiver says what a method may do.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 03 | Build | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

A Python class bundles data, behaviour, inheritance, class attributes and a lot of
convention. When you read `convo.trim()`, you can't tell whether it reads the object, changes
it, or makes it unusable afterwards. You find out by reading the method body, and that's only
true until someone edits it.

Rust splits these apart. A `struct` holds data. An `impl` block holds methods. **Each method's
first parameter states exactly what it does to the value.** There's no inheritance; Phase 03
covers what replaces it.

## Concept

### Three kinds of struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Message { pub role: String, pub content: String, pub tokens: u32 }  // named fields

pub struct ModelName(pub String);    // tuple struct: a named wrapper (the "newtype" pattern)

pub struct Unit;                      // unit struct: no data, used as a marker
```

`#[derive(...)]` generates standard behaviour, like `@dataclass(eq=True, repr=True)`:

| derive | gives you | Python |
|---|---|---|
| `Debug` | `{:?}` printing | `__repr__` |
| `Clone` | `.clone()` deep copy | `copy.deepcopy` |
| `PartialEq` | `==` and `!=` | `__eq__` |
| `Default` | `Type::default()` with zero/empty fields | default field values |

`Display` (`{}` printing, Python's `__str__`) is written by hand with `impl fmt::Display`.

### Methods and the receiver

| First parameter | Meaning | After the call |
|---|---|---|
| *(none)* | associated function, e.g. `Message::new(...)` | — (Python `@classmethod` / `__init__`) |
| `&self` | read-only access | value unchanged |
| `&mut self` | may change the value | value changed, still usable |
| `self` | takes ownership (consumes) | caller can't use the value any more |

`new` is a naming **convention**, not a keyword. Rust has no constructors; a function returning
`Self` is a constructor. **Struct update syntax** `Message { content: x, ..other }` copies the
remaining fields from another value.

### Privacy

Fields are **private to the module** unless marked `pub`. `Conversation` keeps `messages`
private, so the only way to change it is through `push` and `trim_to_budget`, which keep the
budget logic in one place. Python's `_private` is a convention; Rust's is enforced.

### Where the analogy breaks

- **No inheritance.** You can't write `class Admin(User)`. Put a `User` inside an `Admin`
  (composition), and share behaviour through traits (Phase 03).
- **No `None` fields by default.** Every field must have a value. "Maybe missing" is spelled
  `Option<T>` (`model: Option<ModelName>`).
- **Methods don't take `self` implicitly.** The receiver is part of the signature, and it's
  checked: a `&self` method can't modify anything.

## Explore

The example models a chat conversation with a token budget.

```bash
cargo run -p structs
cargo test -p structs
```

Expected output:

```
[small-model] 4 messages, 39/30 tokens
     system: You are a concise assistant.
       user: What is ownership in Rust?
  assistant: Every value has exactly one owner; when the owner goes out of scope, it is dropped.
       user: And borrowing?
trimmed 2 message(s)
[small-model] 2 messages, 11/30 tokens
     system: You are a concise assistant.
       user: And borrowing?
Message { role: "user", content: "hello", tokens: 1 }
equal? false
[<no model>] 0 messages, 0/0 tokens
handed over 2 messages
```

**Break it on purpose.**

1. Uncomment the last line of `main` (`print!("{convo}");`). `into_messages(self)` consumed
   `convo`. Read error `E0382`.
2. Change `pub fn total_tokens(&self)` to modify something (add `self.max_tokens += 1;`).
   Read `E0594`: a `&self` method can't change the struct.
3. Remove `Default` from `#[derive(...)]` on `Conversation`. Two places break. Why both?

## Practice

1. Add a method `fn last_user_message(&self) -> Option<&Message>`. (Hint: `iter().rev().find(...)`.)
2. `role` is a `String`, so `Message::new("usr", ...)` is accepted even though it's a typo.
   Write down how you'd prevent that. (Lesson 04's enums are the answer.)
3. Add a `fn with_system(mut self, prompt: &str) -> Self` method that pushes a system message
   and returns the conversation, so you can write `Conversation::new("m", 100).with_system("...")`.
   Why does it take `mut self` instead of `&mut self`?
4. Write a test that `trim_to_budget` removes the **oldest** non-system message first.

## Check Your Understanding

1. What's the difference between `&self`, `&mut self` and `self`? When would you pick each?
2. Why does Rust have no constructors, and what does `new` really mean?
3. Why keep `messages` private instead of `pub`?
4. What does `#[derive(Clone)]` cost at runtime, and when is it a problem?

## Further Reading

- *The Rust Programming Language*, chapter 5 "Using Structs to Structure Related Data"
- `std::fmt` docs: `Display` vs `Debug`
