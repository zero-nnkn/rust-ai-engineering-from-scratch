# Traits: Python Protocols with teeth

> **Motto:** A trait names an ability. The compiler checks every type that claims it.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 03 · Traits, Generics & Errors | 01 | Build | ~2 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Vector search can score with dot product, cosine similarity or Euclidean distance. The code
that ranks results shouldn't care which. In Python you'd lean on duck typing ("anything with a
`.score()` method") or a `Protocol` for mypy. Both have gaps: duck typing fails at runtime when
a method is missing or misnamed, and a `Protocol` is only checked if you run mypy and every
call site is annotated.

Rust doesn't use inheritance, so traits are *the* way to share behaviour between types. They're
checked everywhere, always.

## Concept

### Defining and implementing a trait

```rust
trait Scorer {
    fn score(&self, a: &[f32], b: &[f32]) -> f32;   // required
    fn name(&self) -> &'static str;                  // required
    fn higher_is_better(&self) -> bool { true }      // provided: a default implementation
}

struct Euclidean;
impl Scorer for Euclidean {
    fn score(&self, a: &[f32], b: &[f32]) -> f32 { ... }
    fn name(&self) -> &'static str { "euclidean" }
    fn higher_is_better(&self) -> bool { false }      // override the default
}
```

| Python | Rust |
|---|---|
| `class Scorer(Protocol): def score(...)` | `trait Scorer { fn score(...); }` |
| `class Dot:` + methods with matching names | `impl Scorer for Dot { ... }`: an explicit claim |
| ABC with a concrete helper method | trait with a **default method** |
| `isinstance(x, Scorer)` at runtime | `T: Scorer` checked at compile time (next lesson) |
| inheriting from a base class | no inheritance; implement traits, compose structs |

An `impl` must provide every required method with exactly the right signature. Forget one and
it doesn't compile. Misspell one and it doesn't compile either: it's not a method of the trait.

### Standard-library traits you'll implement constantly

| Trait | Gives you | Python | Usually |
|---|---|---|---|
| `Debug` | `{:?}` | `__repr__` | `#[derive]` |
| `Display` | `{}`, `.to_string()` | `__str__` | written by hand |
| `Clone` / `Copy` | `.clone()` / implicit copy | `copy` | `#[derive]` |
| `PartialEq` / `Eq` | `==` | `__eq__` | `#[derive]` |
| `PartialOrd` / `Ord` | `<`, sorting | `__lt__` | `#[derive]` or by hand |
| `Hash` | use as a `HashMap` key | `__hash__` | `#[derive]` |
| `Default` | `T::default()` | default args | `#[derive]` or by hand |
| `From<X>` | `T::from(x)` and `x.into()` | alternative constructors | by hand |
| `Iterator` | `for` loops, adapters | `__iter__`/`__next__` | Lesson 05 |
| `Drop` | cleanup | `__del__` (but deterministic) | Phase 02 · Lesson 06 |

### The orphan rule

You can implement **your** trait for **any** type (`impl Describe for [f32]`), and **any** trait
for **your** type (`impl Display for Similarity`). You can't implement someone else's trait for
someone else's type (`impl Display for Vec<f32>`). That keeps two crates from defining
conflicting implementations. The usual workaround is a newtype wrapper (Lesson 07).

### Where the analogy breaks

A trait isn't a base class. There are no fields in a trait, no `super()`, no shared state. A
struct can implement any number of traits, and you can add a trait to a type long after the
type was written (as with `Describe for [f32]`). In Python terms, it's closer to a `Protocol`
that every class must explicitly opt into, and that mypy runs on every line, always.

## Explore

```bash
cargo run -p traits
cargo test -p traits
```

Expected output:

```
dot       scores [ 1.900, 20.000,  0.000] best = doc Some(1)
cosine    scores [ 0.999,  1.000,  0.000] best = doc Some(1)
euclidean scores [ 0.100, 12.728,  1.732] best = doc Some(0)
Display  : 99.9%
From     : 100.0%
Default  : 0.0%
PartialOrd: true
Describe : vec(dim=3, first=Some(1.0))
```

Three metrics, two different winners. Dot product rewards the long vector, cosine says doc 1 points
in *exactly* the same direction, and Euclidean picks the nearest point. The ranking code
(`best_match`) is the same for all three. It's a default method on the trait, and it uses
`higher_is_better` to decide which way to sort.

`main` uses `&dyn Scorer` to put three different types in one array. That's a trait object;
Lesson 03 explains it.

**Break it on purpose.**

1. Delete `fn name(...)` from `impl Scorer for Dot`. Read `E0046`: the compiler lists the missing
   method.
2. Rename `score` to `scores` in `impl Scorer for Cosine`. Read `E0407` ("method is not a member
   of trait") and the `E0046` that comes with it. In Python this would silently fall back to a
   missing method at runtime.
3. Try `impl fmt::Display for Vec<f32> { ... }`. Read `E0117`, the orphan rule.

## Practice

1. Add `struct Manhattan;` (sum of absolute differences) implementing `Scorer`. Lower is better.
   Write a test where Manhattan and Euclidean disagree on the best match.
2. Add a default method `fn rank(&self, query: &[f32], candidates: &[Vec<f32>]) -> Vec<usize>`
   that returns all candidate indexes, best first. Use it from `best_match` so there's one
   sorting rule.
3. Implement `PartialOrd`, `Ord`, `Eq` by hand for a `Score(u32)` newtype that sorts **descending**,
   and sort a `Vec<Score>` with `.sort()`. When would you do this instead of `sort_by`?
4. Implement `From<&str> for Similarity` that parses a percentage like `"87.5%"`. Should a bad
   input panic or return something else? (Hint: look at the `TryFrom` trait.)

## Check Your Understanding

1. What's the difference between a required and a provided (default) trait method?
2. What does `impl From<f32> for Similarity` give you, besides `Similarity::from`?
3. What does the orphan rule forbid, and why?
4. How is implementing a trait different from inheriting from a Python base class?

## Further Reading

- *The Rust Programming Language*, chapter 10.2 "Traits: Defining Shared Behavior"
- `std::fmt::Display`, `std::convert::From` docs
