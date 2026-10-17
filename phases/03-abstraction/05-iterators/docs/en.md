# Iterators: lazy pipelines like generators

> **Motto:** Describe the pipeline; nothing runs until something asks for a value.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 05 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Data prep for AI is pipelines: read rows, drop bad ones, dedupe, transform, window, batch. In
Python you choose between list comprehensions (clear, but each step builds a whole new list in
memory) and generators (lazy, but easy to exhaust by accident, and slower per item). Long chains
of either get hard to read, and nothing tells you when a generator was consumed twice.

Rust iterators are lazy like generators, read like a chain of comprehensions, and compile down
to the same machine code as a hand-written loop. You've used them since Phase 01; this lesson
explains how they work and how to write your own.

## Concept

### One trait, one required method

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;   // Some(value), or None when finished
    // ... plus ~70 provided methods built on `next`: map, filter, take, sum, collect, ...
}
```

Implement `next` and you get every adapter for free. A `for` loop calls `next` until `None`.
That's Python's `__next__` with `StopIteration` replaced by `None`.

### Three ways to iterate a collection

| Call | Yields | After the loop | Python |
|---|---|---|---|
| `v.iter()` / `for x in &v` | `&T` | `v` still usable | `for x in v` |
| `v.iter_mut()` / `for x in &mut v` | `&mut T` | `v` still usable, modified | index loop with assignment |
| `v.into_iter()` / `for x in v` | `T` (owned) | `v` is moved, gone | — |

### Lazy adapters and eager consumers

| Lazy (return another iterator) | Consumers (run the pipeline) |
|---|---|
| `map`, `filter`, `filter_map`, `flat_map` | `collect`, `count`, `sum`, `product` |
| `enumerate`, `zip`, `chain`, `rev` | `fold`, `reduce`, `for_each` |
| `take`, `skip`, `take_while`, `step_by` | `min`, `max`, `min_by_key`, `max_by_key` |
| `inspect`, `peekable`, `scan`, `windows`-like adapters | `any`, `all`, `find`, `position` |
| | `partition`, `unzip`, `last`, `nth` |

Nothing happens until a consumer runs. The test `iterators_are_lazy` proves it: `find` on a
range of a million numbers only ever produces two of them.

### `collect` builds whatever type you ask for

```rust
let v: Vec<u32>         = it.collect();
let s: HashSet<&str>    = it.collect();
let m: HashMap<K, V>    = pairs.collect();
let text: String        = chars.collect();
let all: Result<Vec<u32>, ParseIntError> = strings.map(|s| s.parse()).collect();  // stops at the first Err
```

The last one is a favourite: a list of results becomes a result of a list.

### Where the analogy breaks

- A Python generator can only be consumed once, silently giving nothing the second time. A Rust
  iterator is a value: consuming it takes ownership, so reusing it is a compile error, not a
  silent empty loop. To iterate twice, call `.iter()` twice.
- Python comprehensions are eager and allocate. Rust chains allocate only where you `collect`.
- Rust iterators are a zero-cost abstraction: in `--release` a `filter().map().sum()` chain
  compiles to the same loop you'd write by hand.

## Explore

```bash
cargo run -p iterators
cargo test -p iterators
```

Expected output:

```
cleaned ids   : [1, 5, 6]
total words   : 13
longest id    : Some(5)
any tip?      : true
all end in '.': true
batches       : 2 of sizes [2, 1]
enumerate     : ["0:fact", "1:fact", "2:tip"]
rev/step_by   : [10, 8, 6, 4, 2]
partition     : 2 facts, 1 other
fold          : 1,5,6
windows take 2: ["one two three", "three four five"]
windows count : 3
collect set   : ["five", "four", "seven", "three"]
collect Result: Err (stops at the first bad item)
```

`clean` drops row 2 (empty), row 3 (no label) and row 4 (duplicate text), in one pass, with no
intermediate vectors. Note the third `filter` uses a `HashSet` captured by the closure:
`seen.insert` returns `true` only the first time.

**Break it on purpose.**

1. In `main`, write `let it = cleaned.iter(); let a = it.count(); let b = it.count();`. Read
   `E0382`: `count` consumed the iterator. Compare with a Python generator, where the second
   count would silently be 0.
2. Remove the type annotation from `let words: HashSet<&str> = ...collect();`. Read `E0282`:
   `collect` needs to know what to build.
3. Delete the `.collect()` at the end of `clean` and change its return type to
   `impl Iterator<Item = &Example>`. Read `E0373`: the lazy iterator's closure borrows `seen`, a
   `HashSet` that dies when `clean` returns. (The callers break too: they used `.iter()` on a `Vec`.)
   Fix it with `move` on the closure. Why does `move` work here?

## Practice

1. Write `fn dedupe_by_key<T, K: Eq + Hash>(items: Vec<T>, key: impl Fn(&T) -> K) -> Vec<T>`.
2. Add `fn sentences(text: &str) -> impl Iterator<Item = &str>` that yields sentences lazily
   (split on `.`, `!`, `?`, trim, skip empty), and use it with `take(3)`.
3. Implement `Iterator` for `struct Batches<I> { inner: I, size: usize }` that groups any iterator
   into `Vec`s of `size`. This is the generic version of `chunks` that works on streams, not
   just slices. Test it on a range.
4. Rewrite `clean` as a plain `for` loop with `if`/`continue`. Then build both in `--release` and
   time them on a dataset of a million rows. Which do you find easier to review?

## Check Your Understanding

1. What does an iterator have to implement, and what does it get for free?
2. What's the difference between `iter()`, `iter_mut()` and `into_iter()`?
3. Why does nothing happen until a consumer like `collect` or `sum` runs?
4. How does `collect::<Result<Vec<_>, _>>()` behave when one item is an error?

## Further Reading

- *The Rust Programming Language*, chapter 13.2 "Processing a Series of Items with Iterators" and 13.4 "Comparing Performance"
- `std::iter::Iterator` docs: the provided methods list
