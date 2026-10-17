# Box, Rc, Arc, RefCell, Mutex

> **Motto:** Each smart pointer relaxes exactly one ownership rule, and charges you for it.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 05 | Build | ~2.5 h | `outputs/smart-pointer-decision-chart.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

The ownership rules (one owner, many readers or one writer) cover most code. A few real
situations don't fit:

- A type that contains itself, like a filter expression `A AND (B OR NOT C)`.
- One value that several parts of the program need to keep, with no obvious single owner:
  a shared system prompt, a model config, a tokenizer.
- A method that only reads (`&self`) but must update a statistic, like a cache's hit counter.
- Several threads updating the same state, like a shared token budget.

In Python, every object is reference-counted, mutable from anywhere, and protected (somewhat)
by the GIL, so you never had to choose. In Rust you pick the tool that relaxes exactly the rule
you need, and nothing more.

## Concept

| Type | Relaxes | Cost | Python feeling |
|---|---|---|---|
| `Box<T>` | "size must be known": puts `T` on the heap, one owner | one allocation | every object (always on the heap) |
| `Rc<T>` | "one owner": shared ownership, **read-only**, one thread | reference count | normal Python references |
| `Arc<T>` | same as `Rc`, **across threads** | atomic reference count | — |
| `RefCell<T>` | "`&mut` is checked at compile time": checked **at runtime** instead | a borrow flag; **panics** on violation | mutating through any reference |
| `Mutex<T>` / `RwLock<T>` | same as `RefCell`, **across threads**, by locking | a lock; can block or deadlock | `threading.Lock` |

They combine:

| Combination | Means | Typical use |
|---|---|---|
| `Rc<RefCell<T>>` | many owners, each may mutate (one thread) | trees and graphs with parent links, GUI state |
| `Arc<Mutex<T>>` | many owners across threads, one writer at a time | shared counters, budgets, caches in a server |
| `Arc<T>` (immutable) | many readers across threads, no locking | config, tokenizer, model weights |

### How each one looks

```rust
let f = Filter::Not(Box::new(Filter::Eq("draft", "true")));   // Box: recursion

let prompt = Rc::new(String::from("..."));
let c1 = Rc::clone(&prompt);          // +1 owner, no copy of the string
Rc::strong_count(&prompt);            // how many owners

let hits = RefCell::new(0);
*hits.borrow_mut() += 1;               // runtime-checked &mut

let budget = Arc::new(Mutex::new(1_000));
let b = Arc::clone(&budget);           // give a handle to another thread
*b.lock().unwrap() -= 1;               // lock → guard → unlock when the guard drops
```

`Rc::clone(&x)` is the conventional spelling (rather than `x.clone()`) because it signals "cheap
handle copy", not "deep copy of the data".

### Where the analogy breaks, and when *not* to use these

These are escape hatches, not defaults. `Rc<RefCell<T>>` everywhere is Python written in Rust
syntax: it compiles, but you lose compile-time checking and gain runtime panics. Before reaching
for one, ask:

1. Can one part of the code own it and lend `&` to the rest? (usually yes)
2. Can the data be passed by value through a function, or sent through a channel (Phase 06)?
3. For graphs: can nodes live in a `Vec` and refer to each other by **index**? (Lesson 08)

In Part 2, an agent reaching for `Arc<Mutex<_>>` in single-threaded code is one of the review
smells (Phase 04 · Lesson 05).

## Explore

```bash
cargo run -p smart-pointers
cargo test -p smart-pointers
```

Expected output:

```
Box     : doc_a true / doc_b false
Rc      : 3 conversations, 4 owners of one prompt, same allocation: true
Rc      : "You are a careful Rust reviewer." asks "question 2"
Rc      : after drop, owners = 1
RefCell : cache hits = 2
Arc+Mutex: granted 1000 tokens across 4 threads, 0 left
```

The test `budget_is_never_overspent` runs 8 threads competing for 500 tokens and always gets
exactly 500. Without the `Mutex`, two threads could read the same value and both decrement it.

**Break it on purpose.**

1. Change `Not(Box<Filter>)` to `Not(Filter)`. Read `E0072` ("recursive type has infinite
   size") and its suggestion.
2. In `spend_in_parallel`, replace `Arc` with `Rc` (both the import and the calls). Read `E0277`:
   `Rc` cannot be sent between threads safely. The compiler knows which types are thread-safe.
3. Remove the `Mutex` and try `Arc<u32>` with `*remaining -= 1`. Read the error: `Arc` alone
   gives shared, read-only access.
4. Run the test `refcell_panics_on_overlapping_mutable_borrows` and read it. This is `E0499` from
   Lesson 02, moved from compile time to a runtime panic. That's the trade-off `RefCell` makes.

## Practice

1. Add `Filter::In(&'static str, Vec<&'static str>)` (value is one of a list) and a test.
   Why doesn't this variant need a `Box`?
2. Rewrite `EmbeddingCache` without `RefCell` by making `get` take `&mut self`. What changes for
   callers? When is `RefCell` the better choice, and when is `&mut self`?
3. Replace `Mutex<u32>` in `spend_in_parallel` with `std::sync::atomic::AtomicU32` and
   `fetch_update`. Keep the test passing. Which is simpler here, and why?
4. Write a parent/child structure where children point back to their parent. Try `Rc` both ways
   and count references after dropping the root: it never reaches zero (a leak). Fix it with
   `std::rc::Weak`. Then redo it with a `Vec` of nodes and indexes, and compare.

## Ship It

`outputs/smart-pointer-decision-chart.md`: a decision chart from "I need…" to the right type,
plus the questions to ask before using any of them.

## Check Your Understanding

1. Which ownership rule does each of `Box`, `Rc`, `RefCell` and `Mutex` relax?
2. Why does `Rc` not work across threads, and what does `Arc` do differently?
3. What happens if you violate the borrowing rules through a `RefCell`?
4. Why is `Rc<RefCell<T>>` in every struct a design smell?

## Further Reading

- *The Rust Programming Language*, chapter 15 "Smart Pointers" and chapter 16.3 "Shared-State Concurrency"
- *Rust Atomics and Locks* (Mara Bos), chapter 1, for what `Arc` and `Mutex` do underneath
