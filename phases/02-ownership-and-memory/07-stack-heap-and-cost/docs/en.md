# Stack vs heap & the real cost of clone()

> **Motto:** Moves are cheap, clones are not, and Rust makes both visible.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 07 | Learn | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

In Python, every value is a heap object behind a pointer, and you rarely know what an operation
costs. `b = a` is free, `b = list(a)` copies, `b = a + [x]` copies, `s += "x"` might copy. A slow
pipeline is usually death by a thousand copies nobody can see.

Rust makes cost visible in the code: a **move** is a few bytes copied on the stack, a `.clone()`
is a deep copy you had to type. When you start using coding agents in Part 2, `clone()` is the
first thing to look for in a diff, because agents use it to make the borrow checker go quiet.
This lesson gives you the numbers to judge when that matters.

## Concept

### Stack and heap

| | Stack | Heap |
|---|---|---|
| Holds | fixed-size values: numbers, structs, the *handles* of `String`/`Vec`/`Box` | data whose size is known only at runtime: string bytes, vector elements |
| Allocation | moving a pointer (practically free) | asking the allocator (much slower), freeing later |
| Freed | when the function returns | when the owner is dropped (Lesson 06) |

```
let v: Vec<f32> = vec![0.1, 0.2, 0.3];

   stack                         heap
   v ┌──────────────┐            ┌─────┬─────┬─────┬─────┐
     │ ptr ─────────┼──────────► │ 0.1 │ 0.2 │ 0.3 │  ·  │   (capacity 4)
     │ len = 3      │            └─────┴─────┴─────┴─────┘
     │ cap = 4      │
     └──────────────┘  24 bytes, whatever the length
```

### What each operation costs

| Operation | Copies | Cost |
|---|---|---|
| `let b = a;` (move) | the 24-byte handle | ~nothing |
| `f(&a)` / `&a[1..3]` (borrow, slice) | a pointer (+ length) | ~nothing |
| `a.clone()` | the handle **and every element**, recursively | one allocation per owned piece |
| `v.push(x)` past capacity | the whole buffer into a bigger one | amortised cheap: capacity grows geometrically |
| `Vec::with_capacity(n)` then `push` × n | nothing extra | one allocation |
| `Rc::clone` / `Arc::clone` | a count increment | ~nothing (atomic for `Arc`) |
| `Copy` types (`u32`, `f64`, `&T`, small tuples of these) | the bits | ~nothing; that's why they're `Copy` |

### When `clone()` is fine

- Small values, cold paths (startup, config), or code where clarity matters more than nanoseconds.
- When you genuinely need two independent copies.
- Cloning an `Rc`/`Arc` (cheap by design).

### When it's a smell

- In a loop over large data.
- To silence a borrow error when borrowing would work (Lesson 08 shows the alternatives).
- On every request in a server (allocations add up under load).

### `Cow`: allocate only when needed

`Cow<'_, str>` is either a borrowed `&str` or an owned `String`. A function that *usually* returns
its input unchanged but *sometimes* modifies it can return a `Cow` and allocate only in the
second case: text normalisation, escaping, quote fixing.

## Explore

Always measure performance with `--release` (Phase 00 · Lesson 02).

```bash
cargo run --release -p stack-heap-and-cost
cargo test -p stack-heap-and-cost
```

Expected output (sizes on a 64-bit machine; **your timings will differ**):

```
--- sizes on the stack (bytes) ---
u8                     1
f32                    4
u64 / usize            8 / 8
&str  (ptr+len)       16
&[f32] (ptr+len)      16
String (ptr+len+cap)  24
Vec<f32>              24
Box<[f32; 1024]>       8
[f32; 1024]         4096
Option<Box<u8>>        8  (None uses the null pointer: free)
Option<u64>           16  (needs an extra tag)
--- Vec growth: capacity while pushing 100 f32s ---
[0, 4, 8, 16, 32, 64, 128]
--- move vs clone of 200,000 strings (timings vary by machine) ---
clone     9.490ms
move     54.000ns
same data: true
--- Cow: allocate only when needed ---
"plain text" → "plain text": borrowed (no allocation)
"“smart” quotes" → "\"smart\" quotes": owned (allocated)
```

The clone took about **175,000 times** longer than the move here, for the same data. The exact
growth steps of `Vec` are an implementation detail; the test only checks that capacity at least
doubles each time.

**Break it on purpose.**

1. Run the same program without `--release`. Compare the clone time. Never draw performance
   conclusions from a debug build.
2. In `growth_steps`, replace `Vec::new()` with `Vec::with_capacity(n)`. How many steps are left?
3. Add `let s = String::from("x"); let t = s; println!("{s}");`. This is the move from Lesson 01.
   Now change `String` to `u32` and it compiles. `size_of` explains why `u32` can be `Copy` and
   `String` can't.

## Practice

1. Measure building a 1 MB string three ways: `+=` in a loop, `String::with_capacity` + `push_str`,
   and `parts.join("")`. Use `Instant` and `--release`, and run each a few times.
2. Write `fn escape_prompt(text: &str) -> Cow<'_, str>` that escapes `{` and `}` as `{{` and
   `}}` only if the text contains them. Test both branches with `matches!`.
3. Find every `.clone()` in Phase 01's `word-stats` project and in Phase 02 so far. For each one,
   say whether it's needed, and what it would take to remove it.
4. Predict the `size_of` of `Option<String>`, `Option<Vec<u8>>`, `(u8, u64)` and `[u8; 0]`.
   Check with code. Why is `(u8, u64)` not 9?

## Check Your Understanding

1. Why is moving a `Vec` of a million elements as cheap as moving an empty one?
2. Where are the bytes of a `String` stored, and where is the `String` itself?
3. Why does a `Vec` grow its capacity geometrically instead of by one each time?
4. When is a `clone()` in agent-written code worth flagging in review, and when is it fine?

## Further Reading

- *The Rust Programming Language*, chapter 4.1 "What Is Ownership?" (the stack and heap section)
- `std::borrow::Cow` docs
- *The Rust Performance Book*: "Heap Allocations"
