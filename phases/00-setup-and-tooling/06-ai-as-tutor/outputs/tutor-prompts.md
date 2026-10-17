# Tutor prompts for Part 1

Use these with any AI assistant. Inside this repo, `/learn-rust` already follows these rules.

## Stuck on a compiler error
> Here's my code and the full error. Explain which rule fired, tell the story from the labels
> (where the value was created, moved, borrowed), and point at the line where the real mistake
> is. Give me a hint first, not the fix. I'll try again before asking for the solution.

## Want only a hint
> I'm working on this exercise: <paste>. My attempt and what I think is wrong: <paste>.
> Give me the smallest hint that would unblock me: one sentence, no code.

## After seeing a solution
> I'll explain this solution back in my own words. Tell me what I got wrong or left out,
> then ask me one follow-up question that checks I really understand it.

## Predict, then check
> Before I run this, here's what I predict it prints / whether it compiles: <prediction>.
> Don't tell me if I'm right. Ask me one question that would make me reconsider, if I'm wrong.

## Review my code
> Review this Rust as a senior engineer would, for a learner: correctness first, then
> idioms (unnecessary clone, unwrap that should be ?, &Vec<T> instead of &[T], index loops).
> Don't rewrite it. List findings with line numbers, most important first.

## Quiz me
> Quiz me on <topic>, one question at a time. Mix: predict the output, find the compile error,
> explain why. Wait for my answer before the next question. At the end, list what to review.

## Python bridge
> I understand this in Python: <snippet>. Show me the idiomatic Rust, then tell me exactly
> where the Python mental model breaks.

## Verify a claim
> You said <claim>. Give me a minimal program (under 15 lines) that would prove or disprove it,
> and tell me what output each case would produce. I'll run it.
