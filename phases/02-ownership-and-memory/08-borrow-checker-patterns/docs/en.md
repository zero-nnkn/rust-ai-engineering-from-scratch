# Borrow checker battle patterns

> **Motto:** When the borrow checker says no, it's usually pointing at a design question. Answer the question, don't silence it.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 08 | Learn | ~2.5 h | `outputs/skill-borrow-checker-coach.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

You now know the rules. In real code you'll still hit the same handful of borrow errors again
and again, especially when porting Python habits. There are two bad reactions:

- **Clone until it compiles.** It works, it's slow, and it hides the design problem.
- **`Rc<RefCell<..>>` everything.** It works, you lose compile-time checking, and you get runtime panics.

Coding agents do both, constantly, because both make the error go away. Recognising the seven
patterns below, and the idiomatic fix for each, is what lets you write Rust quickly *and*
review an agent's Rust properly.

## Concept

| # | You wrote (the fight) | Error | Why the compiler refuses | Idiomatic fix |
|---|---|---|---|---|
| 1 | change a `Vec` while iterating over it | E0502 | the iterator holds `&v` while you need `&mut v` | `retain` / `drain` / `iter_mut`; or two phases: collect changes, then apply |
| 2 | keep `&v[0]`, then `v.push(...)` | E0502 | `push` may reallocate, so the reference could dangle | keep an **index** or an id, not a reference |
| 3 | two `&mut` into one `Vec` | E0499 | the compiler can't prove `v[i]` and `v[j]` differ | `split_at_mut`, `swap`, or `get_disjoint_mut` (newer Rust) |
| 4 | loop over `self.items` mutably and call `self.method()` | E0499 | a method call borrows **all** of `self` | borrow the **fields** directly in one function, or make the helper take only the field it needs |
| 5 | return `&str` of a `String` made inside the function | E0515 | the `String` is dropped when the function returns | return the owned value (`String`) |
| 6 | move a field out of `&mut self` | E0507 | it would leave `self` half-empty | `std::mem::take`, `std::mem::replace`, `Option::take` |
| 7 | nodes pointing at each other (trees with parents, graphs) | lifetimes everywhere, or cycles | references can't form cycles safely | an **arena**: nodes in a `Vec`, links are indexes |

### The questions behind the patterns

Most fixes come from answering one of these:

1. **Read phase or write phase?** Split them (patterns 1 and 4). Python lets you interleave
   them; Rust asks you to separate them, and the result is usually clearer code.
2. **Do I need a reference, or just a way to find the thing again?** An index or an id is a
   way to find it again, and survives mutation (patterns 2 and 7).
3. **Who should own this?** If a function creates it, the function returns it owned (pattern 5).
   If a struct is giving it away, leave something valid behind (pattern 6).

### Fixes that are usually wrong

| "Fix" | Why it's suspect |
|---|---|
| `.clone()` to get past the error | fine for small/cold data; a smell in loops and hot paths (Lesson 07) |
| `Rc<RefCell<T>>` | moves the check to runtime; panics instead of compile errors (Lesson 05) |
| adding `'static` | almost never what you meant (Lesson 04) |
| `unsafe` | never the answer to a borrow error in application code |

Sometimes `clone()` *is* the right answer: for a small value, or when you truly need an
independent copy. The skill is knowing which case you're in.

## Explore

Each pattern is a function in `code/src/main.rs`, with the fight described in a comment above it.

```bash
cargo run -p borrow-checker-patterns
cargo test -p borrow-checker-patterns
```

Expected output:

```
1 retain     : [1, 3]
1 two-phase  : [1, 3, 4, 5]
2 index      : first is still #1
3 split      : retries [0, 0, 3, 0, 0]
4 fields     : retry #1 (1), retry #3 (1), retry #4 (4), retry #5 (1), retry #9 (1)
5 owned      : #1: what is a lifetime?
6 take       : took 5, left 0
7 arena      : answer question > search docs > read top hit
7 arena      : root has 2 children
```

Note the one `.clone()` in `main` (making the `Queue`): it's there so the demo can keep using
`tasks` afterwards, and the comment says so. That's what a justified clone looks like.

## Practice

The point of this lesson is to meet each error on purpose. For each pattern:

1. Write the "fight" version from the comment into the function (or into a scratch `main`),
   run `cargo build -p borrow-checker-patterns`, and read the **whole** error.
2. Before looking at the fix, write in `LEARNING.md` which question from "The questions behind
   the patterns" it's asking you.
3. Restore the fix and make sure the tests pass.

Then:

4. **Pattern 4, the subtle version.** Inside `retry_all`, replace `log.push(...)` with a call to
   a new method `self.note(msg)`. Read the error. Then make `note` an associated function that
   takes `log: &mut Vec<String>` instead of `&mut self`. Why does that compile?
5. **Clone audit.** Rewrite `add_followups` using a clone of the whole `tasks` vector to "fix"
   pattern 1 the lazy way. It compiles. Measure it with 100,000 tasks in `--release` against the
   two-phase version.
6. **Arena with removal.** Add `fn remove(&mut self, id: usize)` to `Plan`. Removing from the
   middle of a `Vec` shifts indexes and breaks every link. Solve it with tombstones
   (`Option<Step>`) and write a test.

## Ship It

`outputs/skill-borrow-checker-coach.md`: a skill for your coding agent that, when it hits a
borrow error, names the pattern and proposes the idiomatic fix instead of reaching for `clone()`
or `Rc<RefCell<..>>`. You'll reuse it in Part 2.

## Check Your Understanding

1. Why does `v.push(x)` conflict with an existing `&v[0]`, even though `push` adds to the end?
2. Why does calling `self.method()` inside a loop over `self.items.iter_mut()` fail, while
   touching `self.log` directly works?
3. What does `std::mem::take` leave behind, and why does that make it allowed?
4. Why do arenas with indexes avoid the problems of references in graphs? What do you give up?
5. An agent's diff fixes a borrow error by adding `.clone()` inside a loop. What do you ask it to do instead?

## Further Reading

- *The Rust Programming Language*, chapter 4 (re-read it now, it reads differently)
- `std::mem::take`, `std::mem::replace`, `Option::take` docs
- Niko Matsakis's blog posts on non-lexical lifetimes and two-phase borrows (background reading)
