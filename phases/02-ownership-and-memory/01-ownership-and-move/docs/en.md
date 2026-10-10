# Ownership & Move Semantics

> **Motto:** Every value has exactly one owner. Everything else follows.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 01 | Build | ~2 h | `outputs/prompt-ownership-explainer.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.
This is the lesson you'll lean on most when reviewing agent-written code in Part 2.

## Problem

In Python, this is fine:

```python
docs = load_docs()
cleaned = normalize(docs)   # did normalize mutate docs? copy it? who knows
docs.append(extra)          # is `cleaned` affected? depends on normalize's internals
```

You rely on convention and memory: "does this function mutate its argument?",
"is this a view or a copy?", "who frees this 2 GB buffer, and when?" The garbage
collector hides the cost, and the GIL hides the races. In a long-running agent
process, that is exactly where leaks, surprise mutations and slowdowns come from.

Rust makes the answer to every one of those questions **visible in the function
signature**, and checks it at compile time.

## Concept

Three rules:

1. Every value has exactly **one owner** (a variable).
2. When the owner goes out of scope, the value is **dropped** (freed) immediately.
3. Assigning or passing a value **moves** ownership, unless you **borrow** it (`&` / `&mut`)
   or the type is `Copy` (integers, floats, `bool`, `char`).

| Signature | Meaning | Python analogy |
|---|---|---|
| `fn f(docs: Vec<Document>)` | takes ownership; caller loses `docs` | `del docs` after the call |
| `fn f(docs: &[Document])` | read-only borrow | a function that only reads |
| `fn f(docs: &mut [Document])` | exclusive mutable borrow | mutation, but only one writer allowed |
| `fn f(docs: Vec<Document>) -> Vec<Document>` | move in, move out | functional pipeline step |
| `docs.clone()` | explicit deep copy | `copy.deepcopy(docs)` |

The key insight: **moves are cheap.** Moving a `Vec<Document>` copies three machine
words (pointer, length, capacity). It does not copy the strings. `clone()` is the
expensive one, and Rust makes you write it explicitly so you can see it.

```
let docs = sample();        stack: docs ──► heap: [Doc1, Doc2, Doc3]
let other = docs;           stack: other ──► heap: [Doc1, Doc2, Doc3]
                            stack: docs  (invalid — moved)
```

## Explore

Open `code/src/main.rs`. It implements a small document pipeline with four functions,
one per ownership style:

- `total_chars(&docs)` borrows
- `normalize(&mut docs)` mutably borrows
- `dedup(docs) -> docs` moves in and out
- `consume_batch(docs)` moves in for good

Run it from the repository root:

```bash
cargo run -p ownership-and-move
cargo test -p ownership-and-move
```

Then **break it on purpose.** Uncomment this line in `main`:

```rust
// println!("{}", docs.len());
```

Read the full compiler error. Notice it tells you *where* the move happened and suggests
borrowing or cloning. Rust's compiler is a teacher, not a gatekeeper.

## Practice

Exercises (try each one first, ask your tutor for a hint before a solution, keep the compiler output as evidence):

1. Change `total_chars` to take `Vec<Document>` instead of `&[Document]`. What breaks in `main`, and why?
2. Try to call `total_chars(&docs)` while a `&mut docs` borrow is still in use. Read error `E0502`.
3. Write `fn longest(docs: &[Document]) -> &Document`. Why doesn't it need a lifetime annotation? (Preview of lesson 04.)
4. Replace `backup = docs.clone()` by restructuring the code so no clone is needed.

## Ship It

Save this prompt as `outputs/prompt-ownership-explainer.md` and use it whenever the
borrow checker blocks you:

```text
You are a Rust ownership tutor for a Python developer.
I will paste a Rust compiler error and the relevant code.
1. Explain in plain English which value moved or was borrowed, and where.
2. Draw the ownership state as a small ASCII diagram before and after the failing line.
3. Give two fixes (e.g. borrow vs clone vs restructure) and the trade-off of each.
Do NOT just rewrite my code. Teach me the rule so I can fix it myself next time.
```

## Check Your Understanding

1. Why is moving a `Vec` with a million elements cheap?
2. What is the difference between `&[Document]` and `&Vec<Document>` as a parameter type, and why prefer the first?
3. Why does `let b = a;` leave `a` usable when `a: u32`, but not when `a: String`?
4. How does the "only one `&mut` at a time" rule prevent data races?

## Further Reading

- *The Rust Programming Language*, chapter 4: "Understanding Ownership"
- *Rust by Example*: "Scoping rules"
