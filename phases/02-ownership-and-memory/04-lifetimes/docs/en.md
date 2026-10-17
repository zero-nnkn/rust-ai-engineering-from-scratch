# Lifetimes without fear

> **Motto:** A lifetime doesn't change how long anything lives. It describes which data a reference points into.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 04 | Learn | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Lessons 02 and 03 returned references from functions (`most_similar` returned a `&str` into the
store; `chunk_text` returned slices of its input). That raises a question the compiler must
answer every time: **what does the returned reference point into, and is that data still alive
when the reference is used?**

In Python the garbage collector keeps an object alive as long as anything refers to it, so this
question never comes up, at the cost of a GC and of memory you can't reason about. In C it comes
up and isn't checked: returning a pointer to freed memory is a classic crash. Rust checks it at
compile time. Most of the time it works it out alone. Lifetime annotations are what you write
when it can't.

## Concept

### What a lifetime annotation says

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
```

Read `'a` as **"some region of the program where the borrowed data is valid"**. This signature
says: "the result points into `a` or `b`, so it's only valid while *both* are". Annotations
don't extend or shorten anything; they're a contract the compiler checks at every call site.

```
let winner;
{
    let b = String::from("a bit longer");
    winner = longer("short", &b);   // winner may point into b...
}                                    // ...b is dropped here
println!("{winner}");                // ✗ E0597: `b` does not live long enough
```

### Most of the time you write nothing: elision rules

The compiler fills in lifetimes with three rules:

1. Each reference parameter gets its own lifetime.
2. If there's exactly **one** input reference, the output borrows from it.
   (`fn first_line(text: &str) -> &str` just works.)
3. If there's a `&self` or `&mut self`, the output borrows from `self`.
   (`fn vars(&self) -> Vec<&str>` just works.)

You write lifetimes only when these rules don't decide it: usually **two or more input
references and a returned reference**, or a **struct that holds a reference**.

### Structs that hold references

```rust
struct Template<'a> {
    source: &'a str,
    pieces: Vec<Piece<'a>>,
}
```

This says a `Template` borrows from some text and can't outlive it. The payoff: parsing copies
nothing. Every `Piece` is a view into the original template string.

### `'static`

`&'static str` points into data that lives for the whole program: string literals, which are
part of the compiled binary. `'static` is *not* "use this to make the error go away". If you
find yourself adding `'static` to fix a lifetime error, it's usually the wrong fix.

### When to stop borrowing and own instead

| Situation | Choose |
|---|---|
| short-lived view while the source is clearly alive (parsing, searching) | borrow (`&str`, `Template<'a>`) |
| the result must outlive the input (cached, returned from a request handler, sent to another thread) | own (`String`, `Vec<T>`) |
| you're fighting the annotations in a struct that lives a long time | own, and come back later if profiling says the copy matters |

`render` returns an owned `String` for this reason: the finished prompt goes to an API call and
should not be tied to the template's lifetime.

### Where the analogy breaks

Python has no equivalent because it never frees memory that's still referenced. A Rust
lifetime error is the compiler pointing at a reference that *could* outlive its data: in Python
that would be fine (the GC keeps it alive), in C it would be a use-after-free.

## Explore

```bash
cargo run -p lifetimes
cargo test -p lifetimes
```

Expected output:

```
pieces : [Text("Answer "), Var("question"), Text(" using only "), Var("context"), Text(". Be brief.")]
vars   : ["question", "context"]
render : Answer what is a lifetime? using only the Rust book. Be brief.
missing: Err(MissingVar("context"))
longer : a bit longer
static : You are a helpful assistant.
owned  : Hi Rustacean!
```

The test `pieces_point_into_the_source` proves the zero-copy claim: the variable name `y` is at
byte 3 of the original string.

**Break it on purpose.** All three are the errors you'll meet most.

1. Right after `let template = Template::parse(&template_text);`, add `drop(template_text);`.
   Read `E0505`: you can't free the text while the template still borrows it.
2. Remove the `<'a>` and all `'a`s from `longer`. Read `E0106` and its suggestion. Why can't
   elision decide here?
3. Replace the `longer` line in `main` with the `winner` block from the Concept section. Read
   `E0597`. Then fix it two ways: move `b` out of the inner block, or make `longer` return a `String`.

## Practice

1. Write `fn first_word(text: &str) -> &str` with no annotations. Which elision rule applies?
2. Write `struct Chunk<'a> { doc_id: u32, text: &'a str }` and
   `fn chunks<'a>(doc_id: u32, text: &'a str, size: usize) -> Vec<Chunk<'a>>`. Then try to
   return those chunks from a function that reads the text from a file into a local `String`.
   What has to change, and which of the two options from "When to stop borrowing" would you pick?
3. Change `vars` to return `Vec<&str>` with no lifetime (elision ties it to `&self` instead of
   `'a`). It still compiles. Now write code that keeps the result of `vars()` after the
   `Template` is dropped but while the text is alive. Which version allows it, and why?
4. Ask your tutor to explain `RenderError<'a>`: why does the error type carry a lifetime, and
   what would change if it held a `String` instead? When would you prefer each?

## Check Your Understanding

1. What does `'a` in `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str` promise, in plain words?
2. State the elision rules. When must you write lifetimes yourself?
3. Why is adding `'static` usually the wrong way to fix a lifetime error?
4. When should a struct own its data instead of borrowing it?

## Further Reading

- *The Rust Programming Language*, chapter 10.3 "Validating References with Lifetimes"
- `rustc --explain E0106`, `E0505`, `E0597`
