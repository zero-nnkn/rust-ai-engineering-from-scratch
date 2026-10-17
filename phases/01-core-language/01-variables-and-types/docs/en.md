# Variables, mutability & scalar types

> **Motto:** Types are documentation the compiler checks.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 01 | Learn | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Python has one integer type with unlimited size and one float. That's convenient until a value
crosses a boundary: NumPy's `int32`, a token count serialized to a C library, a byte buffer for a
tokenizer. Then overflow and truncation happen silently, far from where the number was created.

Python names are also always rebindable. Any line can change any variable, so to know a value
you have to read the whole function.

Rust makes both explicit: you choose the size of every number, and you opt in to mutation.

## Concept

### `let`, `mut`, shadowing, `const`

| Rust | Meaning | Python |
|---|---|---|
| `let x = 5;` | immutable binding | `x = 5`, and a promise not to reassign |
| `let mut x = 5;` | mutable binding | normal Python variable |
| `let x = x.trim();` | **shadowing**: a new variable reusing the name, possibly with a new type | reassigning with a different type |
| `const MAX: u32 = 200_000;` | compile-time constant, type required | `MAX: Final = 200_000` |

Immutability by default sounds restrictive, but it means that when you see `let`, you know the
value never changes. Mutation stands out: every `mut` is a place to look when something changes.

### Scalar types

| Type | Size | Use for | Python |
|---|---|---|---|
| `i8 i16 i32 i64 i128` | 1–16 bytes, signed | values that can be negative | `int` |
| `u8 u16 u32 u64 u128` | 1–16 bytes, unsigned | counts, sizes, bytes | `int` |
| `usize` / `isize` | pointer size (8 bytes on 64-bit) | indexes and lengths (`v.len()` is `usize`) | `int` |
| `f32` / `f64` | 4 / 8 bytes | `f64` by default; `f32` for ML weights | `float` |
| `bool` | 1 byte | `true` / `false` | `bool` |
| `char` | 4 bytes | one Unicode scalar value, **not** a byte | 1-char `str` |

Literals can carry underscores and suffixes: `1_000_000`, `255_u8`, `3.0_f32`.

### Where Python intuition breaks

1. **Overflow.** `255_u8 + 1` panics in a debug build and wraps to 0 in a release build. When
   overflow is possible, say what you want: `checked_add` (→ `Option`), `saturating_add` (clamp),
   or `wrapping_add` (wrap on purpose).
2. **No implicit conversions.** `u32 + u64` doesn't compile, and neither does `u32 * f64`.
   You convert explicitly with `f64::from(x)`, `u64::from(x)`, or `TryFrom`.
3. **`as` never fails, so it can lie.** `300_i64 as u8` is `44`; `-1_i32 as u32` is `4294967295`.
   Use `as` only when you know the value fits. Otherwise use `u8::try_from(x)`, which returns a
   `Result`.
4. **Unsigned subtraction.** `3_u32 - 5` would be negative. Use `checked_sub`, or a signed type
   if negative values make sense.

## Explore

```bash
cargo run -p variables-and-types
cargo test -p variables-and-types
```

Expected output:

```
used          : 5596 of 200000
budget        : 8000
checked_add   : None
saturating_add: 255
wrapping_add  : 4
remaining     : Some(50000)
remaining     : None
cost          : $0.0168
as u8         : 300 → 44
-1 as u32     : 4294967295
try_from      : Err(TryFromIntError(PosOverflow))
char          : ữ is 3 bytes in UTF-8
prices        : in $3, out $15 per 1M
worst latency : 210 ms
```

**Break it on purpose.**

1. Remove `mut` from `let mut used`. Read error `E0384` (cannot assign twice to immutable variable)
   and its `help:` line.
2. Change `f64::from(tokens)` in `cost_usd` to just `tokens`. Read the type mismatch (`E0308`):
   Rust won't divide a `u32` by an `f64`.
3. Run `cargo test -p variables-and-types overflow -- --nocapture` and read the panic message
   "attempt to add with overflow". Then run it with `--release`. Why does the test fail now?

## Practice

1. Write `fn tokens_for_chars(chars: usize) -> usize` (≈ 4 characters per token, rounded **up**)
   with tests for 0, 1, 4 and 5 characters. Which type did you use, and why `usize`?
2. A model's price is `$0.15` per million input tokens. Compute the cost of 7 billion tokens with
   `u32` token counts. What happens? Fix it with the smallest type change.
3. Use shadowing to turn `let temperature = "0.7";` into an `f32`, then clamp it to `0.0..=2.0`
   with `.clamp()`. Why is shadowing better here than two names like `temperature_str` and `temperature`?
4. Predict, then check: what are `i8::MIN`, `u16::MAX`, `(-7_i32) / 2` and `(-7_i32) % 2`?
   Which of these differs from Python's answer?

## Check Your Understanding

1. Why does Rust make variables immutable by default? What do you gain when reading code?
2. What's the difference between shadowing a variable and making it `mut`?
3. When is `as` safe, and what should you use when it isn't?
4. Why do `v.len()` and indexes use `usize` instead of `i32`?

## Further Reading

- *The Rust Programming Language*, chapter 3.1 "Variables and Mutability" and 3.2 "Data Types"
- `std` docs for `u32`: browse the `checked_*`, `saturating_*` and `wrapping_*` methods
