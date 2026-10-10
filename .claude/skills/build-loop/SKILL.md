---
name: build-loop
description: Develop Rust with the user in control. Turns the user's idea into a spec they approve, then loops - agent codes one small slice, checks run, the user reviews and decides. Use when the user runs /build-loop, or asks to build or extend something in Rust in Part 2 of this curriculum.
---

# build-loop

The user is the product owner and reviewer; you are the developer. The loop:

**idea → spec (user approves) → one slice of code → checks → user review → user decides → repeat**

The user controls every decision point. Never skip one to save time.

## `/build-loop <idea>`

### 1. Idea → spec
- If the idea is unclear, ask at most 3 short questions. Otherwise go straight to a draft spec.
- Draft the spec and show it:
  - **Goal**: 2–3 sentences.
  - **Public API**: Rust types and signatures (structs, enums, traits, function signatures, error enum).
  - **Tests**: names + one line each; at least one failure/edge case per public function.
  - **Constraints**: crates allowed (default: std only, ask before adding any), no `unsafe`,
    no new `#[allow]`, error handling style, performance or I/O limits.
  - **Slices**: 2–6 ordered steps, each small enough to review in a few minutes.
  - **Open questions / assumptions** you'd otherwise make silently.
- **Stop and wait for the user to approve or edit the spec.** Save the approved spec as
  `SPEC.md` next to the crate's `Cargo.toml`.

### 2. One slice
- Write the tests for this slice first and run them to show they fail for the right reason.
- Implement only this slice. The repo's hooks run fmt + clippy after each edit; fix what they report.
- Run `cargo test -p <package>` and `cargo clippy -p <package> --all-targets -- -D warnings`.
- If the compiler fights you for more than two attempts, stop and explain the problem to
  the user instead of restructuring the design on your own.

### 3. Review packet (then stop)
Present, briefly:
- **What changed**: files and a one-line summary each; diff size.
- **Decisions I made that the spec didn't cover**: every one, so the user can overrule it.
- **Look here first**: the 2–4 spots most worth the user's attention (file:line), especially any
  `clone()`, `unwrap()`/`expect()`, `as` casts, `Arc`/`Mutex`, lifetimes, `unsafe`, new dependencies.
- **Checks**: test and clippy results, verbatim summary lines.
- **One question** that checks the user understands the trickiest part, e.g. "why does
  `parse` borrow here instead of taking ownership?" Answer it only after they try.

Then **stop and wait for the verdict**:
- **accept** → append a row to the "Build loop log" in `LEARNING.md`, then propose the next slice.
- **change X** → apply exactly X as a new iteration, then a new review packet.
- **rethink** → go back to the spec; never change the spec or the tests silently.

### 4. Done
When all slices are accepted: run the full checks, show the final API, and list anything left
out of scope. Suggest one line for `CLAUDE.md` "Gotchas" if the user corrected the same kind of
thing twice.

## `/build-loop review`
Walk the user through the current uncommitted diff (`git diff`) using the review order from
Phase 04 · Lesson 05: boundaries → untrusted input → errors → design → tests. Ask the user what
they notice at each step **before** adding your own findings.

## Rules
- One slice per iteration. Small diffs are what make the user's review real.
- Never mark anything done while tests or clippy fail; report failures honestly.
- Don't add dependencies, `unsafe`, or `#[allow]` without the user's explicit OK.
- Explain Rust choices in terms the user learned in Part 1 (ownership, borrowing, traits,
  `Result`), so every loop also teaches.
