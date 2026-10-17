# Generics & trait bounds

> **Motto:** Write it once for every type that can do what you need, and say exactly what that is.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 02 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

A top-k function, a cache, a batching helper: these are the same code whatever the data is.
Python handles this with duck typing, and `TypeVar` hints if you're careful. The function
works with anything until it's called with something that can't be compared or hashed, and
then it fails at runtime, deep inside.

Rust generics let you write the code once, **state what the type must be able to do** (a trait
bound), and get a compile error at the call site if a type can't. And because the compiler
generates specialised code for each concrete type, generic code runs as fast as hand-written code.

## Concept

### Type parameters and bounds

```rust
fn top_k<T, S, F>(items: &[T], k: usize, score: F) -> Vec<(&T, S)>
where
    S: PartialOrd + Copy,      // scores must be comparable and cheap to copy
    F: Fn(&T) -> S,            // `score` is a function from &T to S (Lesson 04)
```

`T` has no bound: `top_k` never looks inside an item, so any type works. `S` needs exactly the
two abilities the body uses. If you sort with `partial_cmp`, `S: PartialOrd` must be stated, and
the compiler checks the body uses nothing else.

| Python | Rust |
|---|---|
| `T = TypeVar("T")` | `fn f<T>(x: T)` |
| `T = TypeVar("T", bound=SupportsLessThan)` | `fn f<T: PartialOrd>(x: T)` |
| `class Cache(Generic[K, V])` | `struct LruCache<K, V>` |
| a `Protocol` argument | `fn f(e: &impl Embedder)` or `fn f<E: Embedder>(e: &E)` |
| checked by mypy, if run | checked by the compiler, at every call site |

### Three ways to write a bound

```rust
fn embed_all<E: Embedder>(e: &E, texts: &[&str]) -> Vec<Vec<f32>>          // inline
fn embed_all<E>(e: &E, texts: &[&str]) -> Vec<Vec<f32>> where E: Embedder  // where clause
fn describe(e: &impl Embedder) -> String                                   // impl Trait
```

All three mean the same thing here. Use `where` when bounds get long, and `impl Trait` when the
type is mentioned only once.

### Bounds on impl blocks

```rust
impl<K: Hash + Eq + Clone, V> LruCache<K, V> { ... }
```

The cache's methods need to hash, compare and clone keys, so they require it. `V` has no bound,
so any value type works. `LruCache<&str, Vec<f32>>` and `LruCache<u64, String>` are both valid.

### Monomorphization: zero-cost generics

For each concrete type you use, the compiler generates a specialised copy of the generic code.
`embed_all::<HashingEmbedder>` is compiled as if you'd written it by hand for that type, so
there's no runtime lookup. The cost is compile time and binary size. Lesson 03 shows the
alternative, trait objects (`dyn Trait`), which make the opposite trade.

### Where the analogy breaks

Python generics are hints that disappear at runtime. Rust generics are real: the type is part
of the compiled code. That's why a `Vec<String>` and a `Vec<u32>` are different types, why you
sometimes need to tell the compiler which type you mean (`collect::<Vec<_>>()`), and why a
generic function can't be called with a type that lacks the bound.

## Explore

```bash
cargo run -p generics
cargo test -p generics
```

Expected output:

```
top_k by len : [("ownership", 9), ("borrowing", 9)]
top_k f64    : [(301.2, 301.2)]
embedder with dim 8
cache hello : sum 5
cache world : sum 5
cache hello : sum 5
cache rust  : sum 4
cache world : sum 5
hits/misses  : 1/4
LruCache<u64, String>: answer
embed_all    : 2 vectors of dim 8
```

Trace the cache by hand with capacity 2: `hello` and `world` are misses, `hello` is a hit (and
becomes most recent), `rust` evicts `world`, and `world` misses again.

**Break it on purpose.**

1. Remove `PartialOrd + ` from the `where` clause of `top_k`. Read `E0599`: the body uses
   `partial_cmp`, which the bound no longer promises.
2. Make a cache keyed by `f32`: `LruCache::<f32, u8>::new(2).get_or_insert_with(1.0, || 0)`.
   Read the error: `f32` doesn't implement `Eq` or `Hash` (NaN isn't equal to itself). In Python,
   `{float("nan"): 1}` silently works, and can't find its own key back.
3. Score words alphabetically: `top_k(&words, 2, |w| w.to_uppercase())`. A `String` can be
   compared, so `PartialOrd` is fine. Read `E0277`: `String` isn't `Copy`. Should you loosen the
   bound to `Clone`, or is there a reason `top_k` asks for `Copy`?

## Practice

1. Write `fn batches<T: Clone>(items: &[T], size: usize) -> Vec<Vec<T>>`. Then write it again
   without the `Clone` bound, returning `Vec<&[T]>`. Which is better, and why does one need the bound?
2. Add a `fn get(&mut self, key: &K) -> Option<&V>` to `LruCache` that also counts hits and
   misses. What bound does it need that `new` doesn't?
3. Write a second embedder, `LengthEmbedder`, and call `embed_all` with both. Then write
   `fn compare<A: Embedder, B: Embedder>(a: &A, b: &B, text: &str) -> bool` that checks they agree
   on the dimension.
4. Make `top_k` take `impl IntoIterator<Item = T>` instead of `&[T]` so it works on any iterator.
   What changes in the return type?

## Check Your Understanding

1. Why does `top_k` need `S: PartialOrd` but nothing for `T`?
2. What's the difference between `fn f<E: Embedder>(e: &E)` and `fn f(e: &impl Embedder)`?
3. What is monomorphization, and what does it cost?
4. Why can't `f32` be a `HashMap` key, and what does that say about Rust's bounds compared to
   Python's runtime checks?

## Further Reading

- *The Rust Programming Language*, chapter 10.1 "Generic Data Types" and 10.2 "Traits" (trait bounds)
- `std::hash::Hash`, `std::cmp::PartialOrd` docs
