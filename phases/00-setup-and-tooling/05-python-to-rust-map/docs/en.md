# Python → Rust mental map

> **Motto:** Most of what you know transfers. Learn exactly where it doesn't.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 00 · Setup & Tooling | 05 | Learn | ~1 h | `outputs/python-to-rust-cheatsheet.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

After four years of Python you think in comprehensions, dicts, dataclasses, `try/except`,
`Optional`, context managers and duck typing. When you start Rust, the instinct is to
translate line by line, and that's where people get stuck: some Python ideas map almost
one-to-one, a few map to something *better*, and a few don't exist at all and need a
different design.

This lesson is a map. You won't master anything here. You'll learn to recognise the shapes
so the next phases feel familiar, and you'll know where to expect trouble.

## Concept

### Maps almost one-to-one

| Python | Rust | Lesson |
|---|---|---|
| `@dataclass` | `struct` + `#[derive(Debug, Clone, PartialEq)]` | 01-03 |
| methods, `self` | `impl Doc { fn f(&self) }` | 01-03 |
| `__str__` / `__repr__` | `impl Display` / `#[derive(Debug)]` | 01-03 |
| `list` | `Vec<T>` | 01-06 |
| `dict`, `Counter` | `HashMap<K, V>` + `entry().or_insert()` | 01-06 |
| `set` / `deque` | `HashSet<T>` / `VecDeque<T>` | 01-06 |
| comprehension | `.iter().filter().map().collect()` | 03-05 |
| generator | an iterator (lazy, like a generator) | 03-05 |
| `enumerate`, `zip`, `sum`, `any`, `all` | same names, as iterator methods | 03-05 |
| f-string | `format!("{x} {y:.2}")` | 01-01 |
| `lambda` | closure `\|x\| x + 1` | 03-04 |
| `match`/`case` (3.10+) | `match`, but it must cover every case | 01-04 |

### Maps to something stricter, and usually better

| Python | Rust | What changes |
|---|---|---|
| `Optional[T]` | `Option<T>` | you can't use the value without handling `None` |
| `raise` / `try/except` | `Result<T, E>` + `?` | errors are part of the function's type; no surprise exceptions |
| type hints | types | checked at compile time, always |
| `Protocol` / ABC | trait | checked at compile time; no inheritance of data |
| `Literal[...]` + per-case fields | `enum` with data | each variant carries its own fields; `match` must handle all |
| `with open(...)` | scope + `Drop` | cleanup happens when the owner goes out of scope, every time |

### Doesn't exist, so you design differently

| Python habit | Why it doesn't exist | What you do instead | Lesson |
|---|---|---|---|
| everything is a shared reference | ownership: one owner per value | move, borrow (`&`, `&mut`), or clone explicitly | Phase 02 |
| mutate a list while others hold it | aliasing + mutation is forbidden | one `&mut` *or* many `&`, never both | 02-02 |
| class inheritance | no inheritance of data | composition + traits | 03-01 |
| `None` as a default for anything | no null | `Option<T>`, or `Default` | 01-05 |
| monkey-patching, `getattr` magic | static types | traits, enums, generics | Phase 03 |
| GIL-protected shared state | no GIL, real parallel threads | `Arc<Mutex<T>>`, channels, or no sharing | Phase 06 |

The first row is the big one. Phase 02 is the longest part of Part 1 for that reason.

## Explore

`code/src/main.rs` puts each idea next to its Python version (in comments). Run it:

```bash
cargo run -p python-to-rust-map
```

Expected output:

```
comprehension : ["RUST IS FAST", "RUST HAS NO GC"]
counts['rust']: 2
enumerate     : 0 Doc#1 (0.81): rust is fast
sum           : 2.37
best          : Doc#3 (0.92): rust has no gc
parse   0.5 : ok 0.5
parse   abc : error: not a number: abc
parse   1.7 : error: out of range: 1.7
match         : user said "hi"
match         : assistant said "hello" (3 tokens)
match         : tool search FAILED
with-block    : wrote file = true
```

**Break it on purpose.** In the `match m { ... }` block, delete the `Message::Tool { name, ok: true }`
arm. Read the error: the compiler lists exactly which case you forgot (`E0004`). Python's
`match` would silently do nothing for that message.

## Practice

1. Add a `Message::System(String)` variant to the enum. Don't touch anything else, and run
   `cargo build -p python-to-rust-map`. Where does the compiler send you? Fix it.
2. Write the Rust version of `[w for d in docs for w in d.text.split() if len(w) > 3]`.
   (Hint: `flat_map` and `filter`.)
3. In `parse_score`, delete the `?` after `.map_err(...)`. Read the type error. What is the
   `?` doing, in Python terms?
4. Write one Python habit of yours that you expect to fight Rust over, and why. Revisit the
   note at the end of Phase 02.

## Ship It

`outputs/python-to-rust-cheatsheet.md`: the three tables above in a printable form, plus syntax
equivalents. Keep it open while you work through Part 1.

## Check Your Understanding

1. Why does Rust need `Option<T>` when it could just allow `null`?
2. What does `?` do, and what's the closest Python equivalent?
3. Why can't you mutate a `Vec` while another part of the code holds a reference into it?
4. Name one Python idea that maps one-to-one, one that becomes stricter, and one that disappears.

## Further Reading

- *The Rust Programming Language*, chapters 3, 5, 6 (skim now, read properly in Phase 01)
- *Rust for Python Programmers*-style comparisons: useful, but test every claim with the compiler
