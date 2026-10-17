---
name: borrow-checker-coach
description: When a Rust borrow-checker error appears (E0499, E0502, E0505, E0506, E0507, E0515, E0597, E0716), identify which of seven known patterns it is and apply or propose the idiomatic fix instead of cloning or adding Rc/RefCell. Use while writing or reviewing Rust.
---

# Borrow checker coach

When the compiler reports a borrow or lifetime error, do not reach for `.clone()`,
`Rc<RefCell<_>>`, `'static` or `unsafe` first. Classify the error, then fix the design.

## 1. Classify

| Pattern | Typical error | Signature in the code |
|---|---|---|
| P1 mutate while iterating | E0502 | `for x in &v { v.push/remove(..) }` |
| P2 reference kept across mutation | E0502 | `let r = &v[i]; v.push(..); use(r)` |
| P3 two `&mut` into one collection | E0499 | `let a = &mut v[i]; let b = &mut v[j];` |
| P4 method call borrows all of `self` | E0499 / E0502 | `for x in self.a.iter_mut() { self.helper() }` |
| P5 returning a reference to a local | E0515 / E0106 | `fn f(..) -> &str { &format!(..) }` |
| P6 moving out of a borrow | E0507 | `let x = self.field;` with `&mut self` |
| P7 cyclic / parent links | lifetimes everywhere, E0597 | `struct Node<'a> { parent: &'a Node }` |
| P8 value dropped while borrowed | E0505 / E0597 | `drop(x)` or scope end while a borrow lives |

## 2. Fix

- **P1:** use `retain` / `drain` / `iter_mut`, or two phases (collect changes, then apply).
- **P2:** keep an index or id instead of the reference; re-look-up after mutating.
- **P3:** `split_at_mut`, `swap`, `get_disjoint_mut`, or restructure to touch one at a time.
- **P4:** borrow the fields separately inside the function, or change the helper to take only the
  field(s) it needs (an associated function or a free function).
- **P5:** return an owned value (`String`, `Vec<T>`); or take the buffer as a parameter.
- **P6:** `std::mem::take`, `std::mem::replace`, or `Option::take`.
- **P7:** an arena: store nodes in a `Vec`, link by index (`usize` or a newtype id). Use `Weak`
  only if shared ownership is genuinely required.
- **P8:** shorten the borrow (copy out the small part you need) or extend the owner's scope.

## 3. Report

When explaining to the user:
1. Name the pattern and the error code.
2. Say which value is borrowed, where the borrow starts, and where it conflicts (from the labels).
3. Give the idiomatic fix, and one alternative with its trade-off.
4. If you do use `.clone()`, say why it's acceptable here (small value, cold path, or a
   genuinely independent copy). Never clone inside a loop over large data just to compile.
