# The Agent Loop

> **Motto:** An agent is a loop. Understand the loop before you build on it.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 10 · Agent Engineering | 01 | Build | ~3 h | `outputs/skill-agent-loop.md` |

**Part 2 · Build with a coding agent.** You read a reference implementation, then extend it
through the build loop.

**Prerequisites:** Part 1 (traits, trait objects, `Result`) and Phase 04. Phase 9 · Lesson 01
is optional: this lesson runs offline with a scripted model.

## Problem

You've used LangChain, LangGraph or an SDK's tool runner in Python. They work until they
don't: a tool throws and the whole run dies, the agent loops forever and burns tokens, or a
trace is impossible to read. When that happens you need to know what the framework does
underneath. Underneath, every agent framework is the same ~40 lines.

## Concept

```
            ┌──────────────────────────────────────────┐
            │                                          │
 user ──► history ──► LLM ──► tool calls? ──yes──► run tools ──► append results
                               │
                               no
                               ▼
                         final answer
```

Four design decisions make the loop robust, and each maps to a Rust idea from Part 1:

| Decision | Rust feature | Why it matters |
|---|---|---|
| The model is swappable | `trait Llm` | Test offline with a scripted model, swap in a real API later |
| Tools share one interface | `trait Tool` + `Box<dyn Tool>` | A registry of different tools, like a dict of callables |
| Tool failures are data | `Result<String, String>` → `ToolResult { is_error }` | The model can read the error and retry; the run doesn't crash |
| Runs are bounded | `max_steps` → `Err(StepLimitExceeded)` | No infinite loops, no surprise bills |

Messages are an `enum`. In Python you'd have dicts with a `"role"` key and hope every branch
handles every role. In Rust, `match` forces you to handle `User`, `Assistant` and `ToolResult`,
and the compiler points to every place that needs updating when a variant is added.

### Explore the reference implementation

`code/src/main.rs` is a complete, tested loop with no dependencies. Read it top to bottom with
your tutor; it's ordered the way you'd design it: messages → `Llm` → `Tool` → registry → `run()`
→ a scripted model.

```bash
cargo run -p agent-loop
cargo test -p agent-loop
```

Expected trace:

```
user: How many words are in 'agents are just loops', times 25?
assistant: I'll count the words, then multiply.
  → call c1 word_count({"text": "agents are just loops"})
  ← result c1 [ok]: 4
  → call c2 calculator({"a": "4", "b": "25", "op": "*"})
  ← result c2 [ok]: 100
assistant: The sentence has 4 words; 4 × 25 = 100.
done in 3 steps
```

The four tests are the real contract: the demo completes; tool errors go back to the model
instead of crashing; unknown tools are errors; runaway agents hit the step limit. Before moving
on, you should be able to explain why `Llm::complete` takes `&mut self` but `Tool::call` takes `&self`.

## Your Idea → Spec

Give the agent its first real-world tool: running a shell command. It's the first tool in the
course that touches the actual system, which is why the spec is yours.

Start your spec from this, and make it your own:

```rust
pub struct Shell {
    pub allowed: Vec<String>,          // program names allowed, e.g. ["ls", "cat", "cargo"]
    pub timeout: std::time::Duration,
}
impl Tool for Shell { /* the agent writes this */ }
```

Tests the spec must include:
1. an allowed command returns its stdout;
2. a program not in `allowed` returns an error result (the process is never started);
3. a command that runs past the timeout is killed and returns an error;
4. stderr and a non-zero exit code are reported to the model, not dropped.

Constraints: std only, no `unsafe`, no `#[allow]`, and the command is **never** passed through
a shell interpreter (`sh -c`).

## The Loop

Run `/build-loop` with your spec. Keep slices small, for example: allow-list check →
spawn and capture output → timeout and kill → exit codes and stderr.

| # | Agent did | I found in review | I asked for |
|---|---|---|---|
| 1 | | | |

More loops once `Shell` is accepted, each with your own spec:
- **Token budget:** `run()` gets a `max_chars` budget over the whole history and a new
  `AgentError::BudgetExceeded`. Notice how the compiler finds every `match` that needs updating.
- **Parallel tool calls:** when the model returns several calls in one message, run them on
  threads with `std::thread::scope`. What trait bound does `Tool` now need? (`Sync`. Know why
  before you accept the diff.)
- **Real model:** after Phase 9 · Lesson 01, implement `Llm` for your API client. The loop
  itself should not change. If the agent changes it, ask why.

## Review Focus

- **Argument splitting:** is `"ls; rm -rf ~"` really blocked, or just split differently?
- **Timeouts:** is the child process killed *and* waited on (reaped), or left as a zombie?
- **Errors as data:** does every failure become a `ToolResult { is_error: true }`, with no `unwrap` and no panic?
- **Tests:** would the tests fail if the allow-list check were deleted?

## Ship It

`outputs/skill-agent-loop.md` is a `SKILL.md`-style file for any coding agent that reads skills.
It tells the agent how to scaffold this loop in a new Rust project with the same four guarantees.
Add your `Shell` tool's safety rules to it once your loop is accepted.

## Check Your Understanding

1. Why does `run()` return `Result<RunResult, AgentError>` while tools return `Result<String, String>`? Who handles each error?
2. What does `Box<dyn Tool>` cost compared to a generic `T: Tool`, and why can't the registry use generics here?
3. In Python, what would happen in this loop if a tool raised an exception? What's the Rust equivalent of forgetting to catch it?
4. Which line in the agent's `Shell` implementation did you have to understand most carefully, and why?
