# Vec, HashMap, HashSet, VecDeque

> **Motto:** Pick the collection by the operation you do most, and say what's inside it.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 06 | Build | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Python's `list`, `dict`, `set` and `deque` hold anything, so a list of document IDs can quietly
gain a string, and `dict` lookups raise `KeyError` at runtime. Python also hides costs:
`list.pop(0)` is O(n), and iteration order is a detail most people never think about until a
test fails on another machine.

Rust's standard collections are the same four ideas, with the element types fixed, safe lookups
that return `Option`, and costs that are documented and predictable.

## Concept

| Python | Rust | Typical use in AI code |
|---|---|---|
| `list` | `Vec<T>` | documents, embeddings, tokens |
| `dict` | `HashMap<K, V>` | inverted index, caches, counters |
| `dict` (sorted output) | `BTreeMap<K, V>` | stable reports, vocabularies |
| `set` | `HashSet<T>` | stopwords, dedup, "seen" checks |
| `collections.deque` | `VecDeque<T>` | sliding windows, queues of jobs, recent messages |
| `collections.Counter` | `HashMap<T, usize>` + `entry` | word counts |
| `collections.defaultdict(list)` | `entry(k).or_default().push(v)` | grouping |

### Safe access returns `Option`

| Operation | Panics if missing | Safe version |
|---|---|---|
| `v[i]` | yes | `v.get(i)` → `Option<&T>` |
| `map[&key]` | yes | `map.get(&key)` → `Option<&V>` |
| `v.first()`, `v.last()`, `v.pop()` | — | already `Option` |

Index with `[]` only when you *know* the index is valid; otherwise use `get`.

### The entry API

```rust
*counts.entry(word).or_insert(0) += 1;          // Counter
postings.entry(word).or_default().push(doc_id);  // defaultdict(list)
```

It looks up the key **once** and gives you a place to read or write. In Python you'd write
`d[k] = d.get(k, 0) + 1`, which looks up twice.

### Where the analogy breaks

- **`HashMap` iteration order is unspecified** and changes between runs. Python dicts keep
  insertion order. If the output order matters, sort the results or use a `BTreeMap`. The
  `search` function sorts for this reason, and the test `ranking_is_deterministic` checks it.
- **Collections own their elements.** `v.push(s)` moves `s` into the vector (Phase 02 explains
  this). Getting an element back gives you a *reference* (`&T`) unless you remove it.
- **No mixed types.** A `Vec<usize>` holds only `usize`. For "one of several kinds", use a
  `Vec` of an enum (Lesson 04).

## Explore

The example is a tiny retrieval index: documents in a `Vec`, an inverted index in a `HashMap`,
stopwords in a `HashSet`, and a short-term memory of recent queries in a `VecDeque`.

```bash
cargo run -p collections
cargo test -p collections
```

Expected output:

```
docs          : 4
docs[0]       : Rust is a systems language
get(10)       : None
first two     : ["Rust is a systems language", "The borrow checker enforces ownership"]
search "rust ownership": [(2, 2), (0, 1), (1, 1)]
search "AI": [(3, 1)]
search "the": []
recent        : ["AI", "the"]
postings[rust]: Some([0, 2])
has 'gc'?     : false
vocab (sorted): [("ai", 1), ("borrow", 1), ("borrowing", 1), ("checker", 1), ("enforces", 1)]
shared words  : ["ownership"]
```

Search results are `(doc_id, score)`. Doc 2 contains both query words, so it ranks first.

**Break it on purpose.**

1. Change `index.docs.get(10)` to `index.docs[10]`. It compiles. Run it and read the panic.
   That's an `IndexError` you could have handled.
2. In `search`, delete the `ranked.sort_by(...)` line and run `cargo test -p collections` a few
   times. Does `ranking_is_deterministic` always pass? Why not?
3. Try `index.docs.push(42);`. Read the type error.

## Practice

1. Add `fn remove(&mut self, id: usize)` that removes a document's ID from every posting list.
   Should `docs` keep a placeholder or shrink? What happens to the other IDs if it shrinks?
2. Score with TF-IDF instead of word counts: store term counts per document
   (`HashMap<String, Vec<(usize, u32)>>`) and use `ln(N / df)` for IDF. Keep the test for
   deterministic ordering.
3. Write `fn top_words(&self, n: usize) -> Vec<(&str, usize)>`: the `n` most frequent words
   across all documents, ties broken alphabetically.
4. Replace `VecDeque` with a `Vec` and `remove(0)`. The tests still pass. Explain in one sentence
   why it's worse with 100,000 recent queries.

## Check Your Understanding

1. Why does `v.get(i)` return an `Option`, while `v[i]` can panic?
2. What does the `entry` API save compared to `get` followed by `insert`?
3. When should you use `BTreeMap` instead of `HashMap`?
4. Why is `VecDeque::pop_front` cheap and `Vec::remove(0)` expensive?

## Further Reading

- *The Rust Programming Language*, chapter 8 "Common Collections"
- `std::collections` module docs: the "When Should You Use Which Collection?" section and the cost table
