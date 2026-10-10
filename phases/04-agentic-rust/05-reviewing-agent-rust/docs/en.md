# Reviewing Agent-Written Rust: The Smells

> **Motto:** Green tests and a clean clippy are where review starts, not where it ends.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 04 · The Build Loop | 05 | Build | ~2 h | `outputs/skill-rust-review.md` |

**Part 2 · Build with a coding agent.** This lesson trains the review step of the loop: you don't write code, you judge it.

## Problem

When an agent writes most of your code, **review becomes your main engineering skill.**
Rust helps a lot: if it compiles, whole classes of bugs (use-after-free, data races, null)
are gone. That's also the trap. Agents learn quickly that "compiles + clippy clean + tests
green" ends the task, and they reach it by the shortest route: `unwrap()`, `clone()`,
`Arc<Mutex<_>>` everywhere, a silenced lint, a test that asserts nothing.

None of those break the build. All of them break production.

## Concept

What the toolchain catches vs. what only you catch:

| Caught automatically | Only caught in review |
|---|---|
| type errors, borrow errors, data races | wrong behaviour that matches weak tests |
| most clippy lints (unless silenced) | `unwrap()`/indexing on untrusted input → panics |
| formatting | errors swallowed with `let _ =` |
| | clones and locks that hide a design problem |
| | `unsafe` justified by a comment, not a proof |
| | silent numeric truncation with `as` |
| | missing input validation (hangs, huge allocations) |
| | tests that assert nothing or only the happy path |
| | a `#![allow(...)]` the agent added to get green |

**The review order that works:**

1. **Diff the boundaries first.** Public signatures, `#![allow]`, `unsafe`, new dependencies.
   These are cheap to read and expensive to get wrong.
2. **Follow untrusted input.** Every `unwrap`, `expect`, `[i]`, `as`, `parse` on data from
   outside (files, network, LLM output, tool arguments).
3. **Follow errors.** Every `let _ =`, `.ok()`, `unwrap_or_default()`: was the error handled or hidden?
4. **Then the design.** Does ownership match the problem (`String` vs `&str`, `Arc<Mutex>` with no threads)?
5. **Read the tests last, adversarially.** What would have to be wrong for every test to still pass?

## Build It

The crate in `code/` was written by an agent from a one-paragraph prompt (it's in the
top-of-file comment). Verify the starting state:

```bash
cargo clippy -p reviewing-agent-rust --all-targets -- -D warnings   # clean
cargo test -p reviewing-agent-rust                                    # 4 passed
```

**Your task:** read `code/src/lib.rs` and write a review in `outputs/my-review.md`.
Find **at least 10** problems. For each: the line, the problem, the consequence, the fix.
Time-box it to 30 minutes, then:

```bash
cargo test -p reviewing-agent-rust -- --ignored
```

`tests/findings.rs` encodes four findings as tests. They should fail; did your review predict
all four?

<details>
<summary><b>Answer key (open after your review)</b></summary>

| # | Where | Problem | Consequence |
|---|---|---|---|
| 1 | `#![allow(clippy::needless_range_loop)]` | Agent silenced a lint instead of fixing it | Hides the index loops below; "clippy clean" was faked |
| 2 | `Arc<Mutex<Vec<Chunk>>>` | No threads anywhere; `&mut self` methods already guarantee exclusivity | Lock overhead, `.lock().unwrap()` everywhere, misleading design |
| 3 | `seen: Vec<String>` + `contains` | O(n) membership check per chunk | Ingesting N chunks is O(N²); use a `HashSet` |
| 4 | `seen` is global | Dedup across *all* documents, not per document | Identical passages in doc B silently lose their attribution (`findings.rs` test 1) |
| 5 | `max_tokens` unvalidated | `max_tokens = 0` → `end == i` forever | Infinite loop: a hung worker (test 2) |
| 6 | `add_document(String, String)` | Takes owned strings it only reads | Forces callers to allocate/clone; take `&str` |
| 7 | `words: Vec<String>` | Allocates a `String` per word | Use `split_whitespace()` slices (`&str`), no copies |
| 8 | `search(&self, query: String)` + `.clone()` of the whole store | Clones every chunk on every search | Memory and time scale with store size per query |
| 9 | `load_line`: `parts[1]`, `parts[2]`, `.parse().unwrap()` | Indexing and unwrap on untrusted file input | One bad line panics the process (test 3); return `Result` |
| 10 | `save`: `let _ = fs::write(...)` | Error swallowed, signature can't report it | Data loss with no signal; the `save_works` test "passes" writing to a non-existent directory |
| 11 | `token_total() -> u32` with `as u32` | Silent truncation | Wrong totals on large corpora (test 4); return `usize` or use `u64::try_from` |
| 12 | `first_chunk_text`: `unsafe { get_unchecked(0) }` | The `SAFETY` comment is a wish, not an invariant | Undefined behaviour on an empty store; use `.first()` and return `Option<&str>` |
| 13 | Tests | Happy-path only; `save_works` asserts nothing; no test for empty input, bad lines, two documents | The suite can't fail for the bugs above |
| 14 | Docs | Dedup behaviour, ordering, and the index format's escaping (tabs/newlines in text) are undocumented | `export()` produces a file `load_line` can't read back if text contains `\t` or `\n` |

</details>

## Use It

1. **Fix it through the build loop.** Run `/build-loop` with your review as the spec, plus this instruction:
   *"Fix exactly these findings. First remove the `#[ignore]` lines in `tests/findings.rs`
   and make them pass. Return `Result` with a `thiserror`-style error enum from `load_line`
   and `save`. Do not add `#[allow]`, `unsafe`, or new dependencies."* Then review its diff
   with the same five-step order. Did it introduce anything new?
2. **Round-trip property.** Ask the agent for a test: `export()` then `load_line` on every
   line gives back the same chunks, including text containing tabs. Watch it discover finding 14.
3. **Make the review repeatable.** Install the skill below and run it on the agent's fix.
   Compare its findings with yours. Where did it miss, and where did it over-report?

## Ship It

`outputs/skill-rust-review.md`: a review skill for coding agents that applies the five-step
order and the smell table. Use it as a second reviewer, never as the only one.

## Check Your Understanding

1. Why is a `#![allow(...)]` added by an agent a red flag even when the lint seems pedantic?
2. Which three smells in this crate would also be bugs in Python, and which are Rust-specific?
3. Why does `&mut self` make the `Mutex` redundant here? When would you need it?
4. Write one test that would have caught finding 10 against the original code. What did you have to change in the API first?
