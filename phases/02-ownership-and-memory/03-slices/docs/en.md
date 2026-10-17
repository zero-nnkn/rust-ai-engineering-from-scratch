# Slices & views (zero-copy text)

> **Motto:** Don't copy data to look at part of it. Borrow a view.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 03 | Build | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

RAG pipelines chunk documents, and training pipelines split batches. In Python, every
`text[a:b]` and `batch[a:b]` on a list **copies** the data. Chunk a 50 MB corpus with 20% overlap
and you allocate about 60 MB of new strings, mostly to read them once and throw them away.
NumPy avoids copies with views, but then you have the opposite problem: a view silently shares
memory with its parent, and writing through one changes the other.

Rust slices are views that never copy, and the borrowing rules from Lesson 02 make them safe:
a slice can't outlive the data it points into, and you can't have a writable view overlapping
another view.

## Concept

### A slice is a pointer and a length

```
let text: String = ...;                    text  { ptr, len: 51, cap: 51 } ──► "Slices are views. A view borrows; it never copies."
let chunk: &str = &text[15..35];           chunk { ptr: text.ptr + 15, len: 20 } ──┘ (points into the same bytes)
```

| Type | View into | Made with |
|---|---|---|
| `&str` | text (`String`, literal) | `&s[a..b]`, `s.trim()`, `s.split(...)` items |
| `&[T]` | an array, a `Vec<T>`, another slice | `&v[a..b]`, `&v[..]`, `v.as_slice()` |
| `&mut [T]` | the same, writable | `&mut v[a..b]`, `v.split_at_mut(i)` |

A `&Vec<T>` converts to `&[T]` automatically, which is why functions should take `&[T]` (it
accepts arrays, vectors, and parts of either).

### Range syntax

| Range | Meaning | Python |
|---|---|---|
| `&v[a..b]` | from `a` up to, not including, `b` | `v[a:b]` |
| `&v[a..]` / `&v[..b]` / `&v[..]` | to the end / from the start / everything | `v[a:]` / `v[:b]` / `v[:]` |
| `&v[a..=b]` | including `b` | `v[a:b+1]` |

There are no negative indexes and no steps. Out-of-range slicing **panics**; `v.get(a..b)` returns
an `Option` instead. On `&str`, the range is in **bytes** and must fall on character boundaries
(Phase 01 · Lesson 07).

### Slice methods that replace Python loops

| Method | Gives you | Python |
|---|---|---|
| `chunks(n)` / `chunks_exact(n)` | non-overlapping pieces of `n` | `[v[i:i+n] for i in range(0, len(v), n)]` |
| `windows(n)` | overlapping windows of `n` | `zip(v, v[1:], ...)` / sliding window |
| `split_at(i)` / `split_at_mut(i)` | two views, before and after `i` | `v[:i], v[i:]` (copies) |
| `first()` / `last()` / `get(i)` | `Option<&T>` | `v[0]` (raises on empty) |
| `contains(&x)` / `starts_with(&[..])` | `bool` | `x in v` |
| `partition_point(pred)` / `binary_search(&x)` | position in sorted data | `bisect` |
| `sort()` / `reverse()` / `swap(i, j)` | in place, via `&mut [T]` | `v.sort()` |

### `split_at_mut`: two writable views at once

Lesson 02's rule says only one `&mut` at a time. `split_at_mut` gives you two, because it
**proves** they don't overlap. That's how parallel code processes the two halves of one buffer
without copying (Phase 06).

## Explore

```bash
cargo run -p slices
cargo test -p slices
```

Expected output:

```
chunk 0: "Slices are views. A "
chunk 1: "s. A view borrows; i"
chunk 2: "ws; it never copies."
chunk 1 starts at byte 15 of the original
body          : [2023, 2003, 1037, 3231]
split_at(2)   : [2023, 2003] | [1037, 3231]
first/last    : Some(2023) Some(3231)
split_at_mut  : [10.0, 2.0, 3.0, 40.0, 5.0, 6.0]
row means     : [5.0, 17.0]
moving avg    : [1.5, 2.5, 3.5]
lower_bound 25: 3
contains 30   : true
```

The line "chunk 1 starts at byte 15 of the original" is the proof: the chunk's pointer is
inside the original string. Nothing was copied.

**Break it on purpose.**

1. Make `chunk_text` return chunks of a `String` it creates inside the function:
   `let owned = text.to_uppercase();` and slice `owned` instead of `text`. Read error `E0515`
   (returns a value referencing data owned by the current function). Why must a returned view
   point into something that outlives the function?
2. In `main`, after `let chunks = chunk_text(doc, 20, 5);`, the `doc` is a `&'static str`, so it
   always lives. Change `doc` to a `String` built with `format!`, then add `drop(doc);` before
   the loop over `chunks`. Read the error.
3. Replace `split_at_mut(3)` with two separate borrows: `let left = &mut batch[..3]; let right = &mut batch[3..];`.
   The ranges don't overlap. Why does the compiler still refuse (`E0499`)?

## Practice

1. Write `fn sentences(text: &str) -> Vec<&str>` that splits on `.`, `!` and `?`, trims each
   piece, and drops empty ones, all without copying. Test it on text with `"..."`.
2. Write `fn batches<T>(items: &[T], size: usize) -> Vec<&[T]>` with `chunks`, then a version
   that returns an iterator instead (`impl Iterator<Item = &[T]>`). Which is better for a
   10-million-item dataset?
3. Write `fn normalize_rows(flat: &mut [f32], dim: usize)` that L2-normalizes each row in place
   with `chunks_exact_mut`. What should happen to a trailing partial row? Decide and test it.
4. Write a Python version of `chunk_text` and measure the memory difference on a large file
   (`tracemalloc`). Then explain where the Rust version allocates (hint: one `Vec` of
   pointers and lengths).

## Check Your Understanding

1. What's inside a `&str` or `&[T]`, and why is creating one cheap?
2. Why should parameters be `&[T]` rather than `&Vec<T>`?
3. Why can't a function return a slice of a `String` it created?
4. How does `split_at_mut` give you two `&mut` without breaking the borrowing rule?

## Further Reading

- *The Rust Programming Language*, chapter 4.3 "The Slice Type"
- `std` docs for the `slice` primitive: the `chunks`, `windows` and `split_*` families
