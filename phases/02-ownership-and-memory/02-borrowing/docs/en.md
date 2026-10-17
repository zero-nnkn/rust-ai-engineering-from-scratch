# Borrowing: &T and &mut T

> **Motto:** Many readers or one writer, never both at once.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 02 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Lesson 01 showed that passing a value *moves* it. Moving everything everywhere would be useless:
you'd lose your data after every function call. Most functions only need to *look at* a value,
or change it and give it back. That's borrowing.

Python lets every reference read and write freely, and that's the source of a classic bug:

```python
for doc in docs:
    if needs_split(doc):
        docs.append(split(doc))   # mutating the list you're iterating over
```

Depending on what you do, you get a silent infinite loop, skipped items, or a
`RuntimeError: dictionary changed size during iteration`. In threaded code, the same pattern
is a data race. Rust rules all of these out at compile time with one rule.

## Concept

### Two kinds of reference

| | `&T` (shared borrow) | `&mut T` (exclusive borrow) |
|---|---|---|
| Can read | yes | yes |
| Can write | no | yes |
| How many at once | any number | exactly one, and no `&T` at the same time |
| Create with | `&value` | `&mut value` (the value must be `let mut`) |
| Python analogy | passing an object you promise not to touch | passing an object to a function that mutates it, with a guarantee nobody else is looking |

**The rule: at any moment, a value has either any number of `&` borrows, or exactly one
`&mut` borrow.** That's "aliasing XOR mutation". Data races and iterator invalidation both need
aliasing *and* mutation at the same time, so the rule makes them impossible.

### A borrow lasts until its last use

```rust
let first = &store.vectors[0];      // shared borrow starts
println!("{}", norm(first));        // ...last use: the borrow ends here
store.normalize_all();              // &mut borrow: allowed
```

The compiler tracks where each reference is *last used*, not where its variable goes out of
scope (this is called non-lexical lifetimes). If you add `println!("{first:?}")` after
`normalize_all`, the shared borrow now overlaps the exclusive one, and it won't compile.

### Using references

| Code | Meaning |
|---|---|
| `fn f(v: &[f32])` | borrow to read |
| `fn f(v: &mut [f32])` | borrow to modify |
| `*x /= n;` | dereference: change the value behind a `&mut` |
| `v.iter()` / `v.iter_mut()` | iterate by `&T` / by `&mut T` |
| `for x in &v` / `for x in &mut v` | the same, as a `for` loop |
| method `&self` / `&mut self` | the method borrows the struct (Phase 01 · Lesson 03) |

Methods auto-reference: `store.normalize_all()` means `EmbeddingStore::normalize_all(&mut store)`.
Inside `normalize`, `norm(v)` lends the `&mut` out as a temporary `&` (a **reborrow**). That's
allowed because the `&mut` isn't used while the `&` exists.

### Where the analogy breaks

Python has no way to say "this function only reads its argument" and have it enforced. In Rust,
`&` is that promise, checked by the compiler. A function signature tells you everything a
caller needs to know: `&[f32]` can't change the data, `&mut [f32]` will, and `Vec<f32>` takes it.

## Explore

```bash
cargo run -p borrowing
cargo test -p borrowing
```

Expected output:

```
norms before  : 5.000 1.414
norm after    : 1.000
rust vector   : [0.6, 0.8, 0.0]
best match    : rust (0.990)
kept copy     : Some("rust"), store now has 4 vectors
counter       : 2
```

**Break it on purpose.** Do each one, read the full error (`E0502` or `E0499`), then undo it.

1. After `store.normalize_all();`, add `println!("{first:?}");`. The shared borrow `first` now
   lives across the exclusive borrow. Read how the error labels both borrows.
2. In the `best_id` line, delete `.to_string()` so `best_id` holds a `&str` borrowed from the
   store. Now `store.add(...)` fails. Why does copying the id out fix it?
3. Write the Python bug in Rust: `for v in &store.vectors { store.add("x", v.clone()); }`.
   Read the error: you can't mutate a collection while iterating over it.
4. Write `let a = &mut counter; let b = &mut counter; bump(a); bump(b);`. Read `E0499`.

## Practice

1. Write `fn scale(v: &mut [f32], factor: f32)` and `fn add_into(dst: &mut [f32], src: &[f32])`.
   Can you call `add_into(&mut v, &v)`? Predict, then check. Why does the rule forbid it, and
   what would go wrong if it didn't?
2. Add `fn get_mut(&mut self, id: &str) -> Option<&mut Vec<f32>>` to the store and use it to
   double one vector in place. Then try to call `store.most_similar(...)` while holding the
   `&mut Vec<f32>` it returned.
3. Fix the Python-style bug from break-it 3 properly: collect the new vectors into a separate
   `Vec` first, then add them in a second loop.
4. Write `fn longest_id(store: &EmbeddingStore) -> &str`. Then write a `main` that calls it,
   adds a vector, and prints the result. Reorder the lines until it compiles, and explain why
   that order works.

## Check Your Understanding

1. State the borrowing rule in one sentence. Which two kinds of bug does it prevent?
2. Why does `let first = &v[0]; v.push(1); println!("{first}");` not compile? What could
   `push` do to the memory that `first` points at?
3. When does a borrow end?
4. How does a function signature with `&`, `&mut` or an owned type help a reviewer of
   agent-written code?

## Further Reading

- *The Rust Programming Language*, chapter 4.2 "References and Borrowing"
- `rustc --explain E0502` and `rustc --explain E0499`
