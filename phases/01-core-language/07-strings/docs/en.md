# String vs &str, UTF-8 & text processing

> **Motto:** `String` owns text, `&str` looks at text, and both are always valid UTF-8.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 07 | Build | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Text is most of what AI code handles: prompts, documents, model output, logs. Python's `str`
hides two things you eventually need to know:

- **Who owns the text, and is it copied?** `s[:100]` makes a new string every time. Slicing a
  large document into thousands of chunks copies all of it.
- **Bytes vs characters.** `len("thế giới")` is 8 in Python (code points). Send it to a
  tokenizer or an API with a byte limit and it's 11. Truncate by bytes and you can cut a
  character in half, producing invalid text.

Rust makes both explicit, which is why Rust tokenizers and text pipelines are fast and don't
corrupt non-English text.

## Concept

### Two types for one idea

| | `String` | `&str` |
|---|---|---|
| What | owned, growable text on the heap | a borrowed view into text owned by someone else |
| Python analogy | a `str` you built | a slice/view that doesn't copy (Python has no real equivalent) |
| Create | `String::from("x")`, `"x".to_string()`, `format!(...)` | `"literal"`, `&s`, `&s[0..5]`, `s.as_str()` |
| Mutate | `push_str`, `push`, `+=`, `clear` | no |
| Function parameter | when you need to keep it | **default choice for parameters** |
| Return type | when you build new text | when you return part of an input |

A `&String` automatically converts to `&str`, so a function taking `&str` accepts both.

```
let s = String::from("Xin chào");   stack: s { ptr, len: 9, cap: 9 } ──► heap: X i n ␣ c h a ̀ o (bytes)
let view: &str = &s[0..3];          stack: view { ptr, len: 3 } ──────► same heap bytes "Xin" (no copy)
```

### UTF-8: bytes, chars, and why `s[0]` doesn't compile

| You want | Use | Note |
|---|---|---|
| byte length (API limits, buffers) | `s.len()` | **bytes**, not characters |
| number of characters | `s.chars().count()` | O(n): it has to walk the text |
| the nth character | `s.chars().nth(n)` | `Option<char>` |
| a byte range | `&s[a..b]` | **panics** if `a` or `b` falls inside a character |
| a char-safe cut point | `s.char_indices().nth(n)` | gives the byte index of the nth char |

`s[0]` doesn't compile: indexing by position is ambiguous in UTF-8 (byte? character?), and
Rust won't guess. Note that `chars()` gives Unicode scalar values. A character that people see
as one symbol (an emoji with a skin tone, some accented letters) can be several `char`s. For
display-width or grapheme work you need a crate (`unicode-segmentation`), but you'll rarely
need it for model input.

### Common operations

| Python | Rust |
|---|---|
| `s.split()` | `s.split_whitespace()` |
| `s.split(",")` | `s.split(',')` |
| `s.strip()` | `s.trim()` |
| `s.splitlines()` | `s.lines()` |
| `s.startswith("x")` / `s.removeprefix("x")` | `s.starts_with("x")` / `s.strip_prefix("x")` (→ `Option`) |
| `"x" in s` | `s.contains("x")` |
| `s.find("x")` (−1 if missing) | `s.find("x")` (→ `Option<usize>`, a byte index) |
| `s.lower()` | `s.to_lowercase()` (new `String`) |
| `" ".join(parts)` | `parts.join(" ")` |
| `f"{x:>10}"` | `format!("{x:>10}")` |

Most methods return **iterators** of `&str` that borrow from the original, with no copying until
you `collect()` or `to_string()`.

## Explore

```bash
cargo run -p strings
cargo test -p strings
```

Expected output:

```
&str          : "Rust  is\tfast"
String        : "growable string!"
normalized    : "Rust is fast"
normalized    : "growable string!"
len (bytes)   : 22
chars         : 17
truncate 8    : "Xin chào…"
truncate 50   : "Xin chào thế giới"
bytes[4..7]   : Some([99, 104, 195])
char at 6     : Some('à')
first line    : "Title: Ownership"
contains      : true
find          : Some(37)
strip_prefix  : Some("Ownership")
split         : ["a", "b", "", "c"]
format        :    chunker|12.35   |00042
redacted      : use key sk-*** for prod
```

"Xin chào thế giới" is 17 characters but 22 bytes: `à`, `ế` and `ớ` take more than one byte each.

**Break it on purpose.**

1. Add `println!("{}", &vi[0..7]);` to `main` and run it. Byte 7 is inside `à`, so it panics
   with "byte index 7 is not a char boundary". `truncate_chars` exists to avoid exactly this.
2. Add `let c = vi[0];`. Read the compile error and its note about UTF-8.
3. Change `first_line`'s return type to `String` without changing the body. Read the mismatch,
   then fix it two ways: `.to_string()` (copy) or keep `&str` (borrow). Which is better here?

## Practice

1. Write `fn truncate_bytes(text: &str, max_bytes: usize) -> &str` that returns the longest
   prefix of at most `max_bytes` bytes **without cutting a character**. (Hint: `is_char_boundary`.)
   This is what you need for APIs with byte limits.
2. Write `fn chunk_chars(text: &str, size: usize) -> Vec<&str>` that splits text into slices of
   `size` characters each, **without copying**. Test it on Vietnamese text.
3. `redact_keys` splits on single spaces, so `"key:sk-abc1234567"` and newlines slip through.
   Improve it, add tests for both, and decide what counts as a word boundary.
4. Predict, then check: `"ﬁ".to_uppercase()`, `"ß".to_uppercase()`, and their `.len()`s.
   Why can changing case change the length?

## Check Your Understanding

1. Why should most functions take `&str` instead of `String` or `&String`?
2. Why does `s.len()` return bytes, and when do you actually want bytes?
3. Why doesn't `s[0]` compile, while `&s[0..3]` compiles but can panic?
4. Which of these copy the text: `&s[..5]`, `s.trim()`, `s.to_lowercase()`, `s.split(' ')`?

## Further Reading

- *The Rust Programming Language*, chapter 8.2 "Storing UTF-8 Encoded Text with Strings"
- `std::str` primitive docs: the method list
