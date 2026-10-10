# Setting Up a Coding Agent for a Rust Repo

> **Motto:** An agent is only as good as the instructions and the feedback loop you give it.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 04 · The Build Loop | 02 | Build | ~1.5 h | `outputs/CLAUDE.md` (template) |

**Part 2 · Build with a coding agent.** Sets up the agent side of the build loop (Lesson 01).

## Problem

Open a coding agent in a fresh Rust repo and ask for a feature. You'll typically get code
that compiles, a `clone()` wherever the borrow checker objected, `unwrap()` on everything,
a new dependency you didn't ask for, and "Done! All tests pass" even though it never ran them.

The agent isn't careless. It doesn't know *your* definition of done, your rules, or which
commands prove the work. Every session it starts from zero, unless you write it down.

## Concept

A coding agent works from three inputs you control:

| Input | Where it lives | What it's for |
|---|---|---|
| **Standing instructions** | `CLAUDE.md` (Claude Code) / `AGENTS.md` (most other agents) at the repo root | Commands, rules, definition of done: loaded every session |
| **The task prompt** | your message | *This* change: goal, constraints, acceptance tests |
| **Feedback** | compiler, clippy, tests, hooks (Lesson 03) | Lets the agent check itself without you |

Good standing instructions are short and specific. A rule like *"write idiomatic Rust"*
changes nothing. *"No `unwrap()` on input from files, network or LLM output; return `Result`"*
changes behaviour, and you can check it in review.

The most valuable section grows over time: **Gotchas.** Each time the agent makes the same
mistake twice, add one line. After a few weeks the file is a compact record of how your
codebase differs from what the agent assumes.

## Build It

1. Start a practice repo: `cargo new --lib agent-practice && cd agent-practice && git init`.
2. Copy `outputs/CLAUDE.md` to the repo root and fill in the placeholders. If you also use
   another agent, add `ln -s CLAUDE.md AGENTS.md`.
3. Give the agent this task **without** the file first (rename it temporarily), then **with** it:

   > Add a function `parse_scores(input: &str) -> ... ` that parses lines like `name,score`
   > into a list of (name, score). Make it robust.

4. Compare the two diffs. Count: `unwrap`s, tests, edge cases covered, whether it ran the
   tests, whether it said "done" honestly.

Keep both diffs as evidence in `LEARNING.md`.

## Use It

1. **Tighten one rule.** Find one thing in the "with" diff you'd still reject. Write the rule
   that prevents it, start a fresh session, and rerun the task. Did the rule work?
2. **Plan first.** Ask for a larger feature (a `Scoreboard` with ranking and ties) and require
   a plan first. Edit the plan before approving it, especially the public signatures.
   Notice how much easier the review is when you agreed on the signatures in advance.
3. **Permissions.** Read your agent's permission settings. Allow `cargo check/clippy/test/fmt`
   without prompts; keep `cargo publish`, `git push` and network commands behind approval.

## Ship It

`outputs/CLAUDE.md`: a Rust-specific template with commands, definition of done, Rust rules,
working style and a Gotchas section. Reuse it in every Rust repo. This curriculum's own `CLAUDE.md`
follows the same rules for Part 2.

## Check Your Understanding

1. Why should "definition of done" list commands rather than describe quality?
2. Why does a shorter instructions file often work better than a longer one?
3. Which rules belong in `CLAUDE.md`, and which belong in the task prompt?
4. When the agent breaks a rule anyway, what are your three options, and which one scales?
