# Lesson templates

Two shapes, one per part. Copy the right one into `phases/<NN>-<phase>/<NN>-<lesson>/docs/en.md`
and keep every heading. Worked examples:

- Part 1: [00-01](phases/00-setup-and-tooling/01-dev-environment/docs/en.md), [02-01](phases/02-ownership-and-memory/01-ownership-and-move/docs/en.md)
- Part 2: [04-01](phases/04-agentic-rust/01-the-build-loop/docs/en.md), [04-05](phases/04-agentic-rust/05-reviewing-agent-rust/docs/en.md), [10-01](phases/10-agent-engineering/01-the-agent-loop/docs/en.md)

---

## Part 1 · Learn Rust

```markdown
# <Lesson title>

> **Motto:** <one line you could put on a sticky note>

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| NN · <Phase name> | NN | Build / Learn | ~N h | <cheat sheet, prompt…> |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem
A concrete pain, ideally one you've hit in Python. Show the Python code that hurts.

## Concept
The principle in 3–5 sentences. One diagram or table. The Python analogy, and where it breaks.

## Explore
A small example in `code/` (a Cargo crate). How to run it, the expected output, and one
"break it on purpose" step with the compiler error to read.

## Practice
2–4 small exercises that get harder. At least one makes the compiler complain instructively.

## Ship It   (optional)
A cheat sheet, prompt or note worth keeping, in `outputs/`.

## Check Your Understanding
3–5 questions that test the *why*, not the syntax.
```

---

## Part 2 · Build with a coding agent

```markdown
# <Lesson title>

> **Motto:** <one line>

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| NN · <Phase name> | NN | Build / Learn | ~N h | `outputs/<artifact>` |

**Part 2 · Build with a coding agent.** You bring the idea and the review; the agent writes the code.

## Problem
What we're building, and why it matters in an AI system.

## Concept
The Rust principles and design choices needed to *judge* the agent's code (learn with your tutor).

## Your Idea → Spec
The idea in 2–3 sentences, then the spec: public types and signatures, 2–4 tests (at least one
failure case), constraints (allowed crates, no `unsafe`, error type…).

## The Loop
Run `/build-loop` with the spec. Suggested slices. A table to log each iteration:
| # | Agent did | I found in review | I asked for |

## Review Focus
The 3–5 things to look hardest at in this lesson's code.

## Ship It
The artifact you keep in `outputs/`: CLI, crate, service, MCP server, prompt or skill.

## Check Your Understanding
3–5 questions, including at least one about a line the agent wrote.
```

---

## Lesson checklist

- [ ] any `code/` builds, `cargo clippy -- -D warnings` is clean, `cargo fmt --check` passes
- [ ] Part 1: the example's expected output in the doc matches a real run
- [ ] Part 2: the loop log in `LEARNING.md` has at least one iteration where you changed something
- [ ] artifact saved in `outputs/`
- [ ] "Outline lesson" note removed, `python3 scripts/generate.py` re-run, lesson shows ✅
