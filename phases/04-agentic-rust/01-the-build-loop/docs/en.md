# The Build Loop: Idea → Agent Code → Review → Repeat

> **Motto:** Your idea, the agent's code, your review. Repeat until it's right.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 04 · The Build Loop | 01 | Learn | ~1.5 h | `outputs/skill-build-loop.md` |

**Part 2 · Build with a coding agent.** This is the first lesson of Part 2, and every lesson after
it uses this loop.

## Problem

Working with a coding agent tends to go wrong in one of two ways:

- **Autopilot.** "Build me X." Five hundred lines arrive, the tests are green, you skim and
  accept. A week later something panics in production, and you can't say what the code
  does because you never really read it.
- **Babysitting.** You dictate every line, so the agent saves no time. You could have typed it yourself.

Both lose control. One gives it away; the other never delegates. You want to delegate the
*typing* and keep the *decisions*.

## Concept

### The loop

```
  💡 your idea
      │
      ▼
  📐 spec ◄─────────────────────────────┐   you approve the spec:
      │                                 │   types, signatures, tests, constraints
      ▼                                 │
  🤖 agent codes ONE small slice        │
      │  (hooks: fmt, clippy, tests)    │
      ▼                                 │
  🔍 you review ──── "change X" ───► agent codes again
      │        └──── "spec was wrong" ──┘
      ▼
   ✅ accept → next slice … → 📦 ship
```

### Your four control points

| Control point | What you decide | Why it's yours |
|---|---|---|
| **The spec** | Public types, signatures, tests, allowed crates | Types are the most precise prompt there is; tests define "done" |
| **Slice size** | How much code arrives before you look | Review quality drops sharply on big diffs |
| **The verdict** | Accept, change X, or rethink the spec | The agent proposes; you decide |
| **Ship** | When it's done | Tests pass *and* you can explain every line you keep |

### Why Part 1 matters here

In the review step you need to answer questions like these: Why does this function take
`String` instead of `&str`? Does this `clone()` hide a design problem? Can this `unwrap()`
ever fail on real input? Why is there an `Arc<Mutex<_>>` with no threads?
That is exactly the ownership, borrowing, traits and error-handling understanding from
Part 1. The agent types; your Rust knowledge is how you steer.

### Why Rust suits this loop

- **The compiler checks the agent's work** within seconds of each edit, with exact errors. Phase 04 ·
  Lesson 03 wires that into hooks so the agent fixes its own mistakes before you see them.
- **Types make your spec precise.** `fn rank(&self, q: &Query) -> Result<Vec<Hit>, SearchError>`
  leaves the agent far less room to drift than a paragraph of English.
- **Shortcuts are visible.** `unwrap`, `clone`, `unsafe`, `#[allow]` are greppable in every diff.
- **But compiling isn't correctness.** Logic bugs, panics on bad input, swallowed errors and weak
  tests all compile. That's what your review is for (Lesson 05).

## Your Idea → Spec

Run the loop once, end to end, on something small. Idea:

> A CLI that reads a text file and prints the 10 most frequent words, ignoring case and punctuation.

Set up a scratch crate inside this repo so the hooks apply:

```bash
cargo new --vcs none phases/04-agentic-rust/01-the-build-loop/code --name word-freq
```

Then in your coding agent: `/build-loop` followed by the idea. **Don't approve the first
spec as is.** Check it for:

- a library function separate from `main` (so it's testable), e.g. `fn top_words(text: &str, n: usize) -> Vec<(String, usize)>`
- tests for: empty input, punctuation, mixed case, ties (what order?), `n = 0`
- what happens when the file is missing (an error message and exit code, not a panic)
- std only

## The Loop

Run it slice by slice. For each review packet, before you answer, ask yourself:
**could I explain this to a colleague?** If not, ask the agent to explain, or ask for a simpler version.

Log every iteration in `LEARNING.md` ("Build loop log"):

| # | Agent did | I found in review | I asked for |
|---|---|---|---|
| 1 | | | |

Expect at least 3 iterations. If you accepted everything on the first try, review again: look
at how ties are ordered and what happens with non-ASCII words like "café".

## Review Focus

- Does `top_words` borrow the text (`&str`) or force a copy?
- Are ties ordered deterministically? (`HashMap` iteration order is random.)
- Is the missing-file case an error with a message, or a panic?
- Do the tests cover the failure cases, or only the happy path?

## Ship It

- `outputs/skill-build-loop.md`: the loop as a skill. It's installed in this repo at
  `.claude/skills/build-loop/`; copy it into any project where you want the same workflow.
- Your `word-freq` crate and its loop log are your first Part 2 evidence.

## Check Your Understanding

1. Which of the four control points did you use most in this lesson? Which did you skip, and what did that cost?
2. Why is a function signature a better prompt than a sentence describing the function?
3. Give one bug that compiles, passes clippy, and would only be caught in your review.
4. When should a review end in "rethink the spec" rather than "change X"?
