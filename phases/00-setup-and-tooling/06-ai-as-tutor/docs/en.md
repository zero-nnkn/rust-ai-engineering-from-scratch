# Learning Rust with an AI tutor

> **Motto:** Let the AI explain everything. Let it think for you about nothing.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 00 · Setup & Tooling | 06 | Learn | ~45 min | `outputs/tutor-prompts.md` (+ the `/learn-rust` skill in `.claude/skills/`) |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

An AI assistant can write any exercise in this course in seconds. If you let it, you'll finish
Part 1 quickly and understand very little, because the skill Part 1 builds (reading Rust and
predicting what the compiler will say) only grows when *you* make the prediction and see it
fail.

The opposite mistake wastes the best tutor you've ever had. An AI can explain the same borrow
error five different ways at 2 a.m., draw the ownership state, compare it to Python, and quiz
you, as patiently as you need.

The goal is to get all the explaining without giving away the thinking.

## Concept

### Four rules for Part 1

1. **Predict before you run.** Before `cargo run` or `cargo build`, say (or write) what you
   expect. A wrong prediction is the most valuable moment in a lesson.
2. **Hint before solution.** Ask for the smallest hint that unblocks you. Ask for the full
   solution only after a real attempt, and then explain it back in your own words.
3. **Ask "why", not "fix".** "Why does this move?" teaches you something. "Fix this" only
   teaches you to ask again.
4. **The compiler is the referee, not the AI.** AI tutors are sometimes confidently wrong,
   especially about lifetimes, edition differences and crate APIs that changed. When an
   explanation surprises you, write a five-line program that tests it.

### What the tutor is good and bad at

| Good at | Weaker at — verify |
|---|---|
| explaining a compiler error in plain words | exact lifetime-elision and borrow-checker edge cases |
| Python comparisons | current versions and APIs of third-party crates |
| drawing ownership diagrams | performance claims ("this is faster") without a benchmark |
| generating practice questions and quizzes | anything about the latest Rust release |
| reviewing your code for idioms | |

### The `/learn-rust` skill

This repo includes a tutor skill in `.claude/skills/learn-rust/SKILL.md`. In Claude Code (or any
agent that reads skills), inside this repository:

| Command | What happens |
|---|---|
| `/learn-rust start` | placement questions, then your route in `LEARNING.md` |
| `/learn-rust` | the next lesson: warm-up recall, concept, explore, practice, quiz, progress update |
| `/learn-rust explain` | paste a compiler error; get rule → story → cause → fixes |
| `/learn-rust quiz 01` | eight questions across a phase, with lessons to review |

`CLAUDE.md` tells the agent that in Part 1 it's a tutor: hints before solutions, and explanations
tied to the lessons. In Part 2 it switches roles and writes code inside the build loop, while
you specify and review.

## Explore

1. Open this repository in your coding agent and run `/learn-rust start`. Answer honestly:
   the route it writes to `LEARNING.md` is only as good as your answers.
2. Pick any error from Lesson 03 (or make one: use a moved `String`). Paste it with
   `/learn-rust explain`. Did the explanation follow the rule → story → cause → fixes order?
3. Ask the tutor: *"In Rust, does `let s2 = s1;` copy the string's bytes?"* Then check the answer
   yourself with the ownership lesson's code (Phase 02 · Lesson 01) or a tiny program.

## Practice

1. **Explain-back.** Pick one concept from Lessons 02–05 (doc tests, `&[T]` vs `&Vec<T>`, `?`).
   Explain it to the tutor in three sentences and ask it to grade your explanation and find the gap.
2. **Catch it being wrong.** Ask the tutor three questions you can verify with the compiler
   (for example: "Will this compile?" with a snippet you write). Keep score in `LEARNING.md`. You'll
   trust it more accurately afterwards, both more and less.
3. **Write your rules.** In `LEARNING.md`, under a heading "How I use the tutor", write your own
   version of the four rules above: what you'll allow yourself to ask for, and when.

## Ship It

`outputs/tutor-prompts.md`: copy-paste prompts for the moments that come up in every lesson:
stuck on an error, want a hint, want a quiz, want your code reviewed, want to check your understanding.

## Check Your Understanding

1. Why does predicting the output before running the code help you learn faster?
2. When is asking for a full solution the right call?
3. Name two kinds of claim from an AI tutor that you should verify with the compiler or a benchmark.
4. How will your role change from the tutor in Part 1 to the coding agent in Part 2?
