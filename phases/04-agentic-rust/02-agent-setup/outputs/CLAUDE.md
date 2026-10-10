# CLAUDE.md — <project name>

<!-- Template from Phase 04 · Lesson 02. Copy to your repo root (Claude Code reads CLAUDE.md;
     other agents read AGENTS.md — symlink one to the other). Keep it under ~100 lines:
     every line is loaded into context on every task. Delete these comments. -->

## What this is
<One paragraph: what the crate/service does, who calls it, what "correct" means.>

## Layout
- `src/lib.rs` — public API. Changing a `pub` signature needs my approval.
- `src/<module>.rs` — <one line each for the modules that matter>
- `tests/` — integration tests; `benches/` — criterion benchmarks

## Commands (run from the repo root)
- Check fast: `cargo check --all-targets`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Test: `cargo test --workspace` (single test: `cargo test -p <crate> <name>`)
- Format: `cargo fmt --all`

## Definition of done
A task is done only when ALL of these hold. Don't report success otherwise; report what fails.
1. `cargo fmt --all --check`, clippy with `-D warnings`, and `cargo test --workspace` pass.
2. New behaviour has tests, including at least one failure / edge case (empty, malformed, boundary).
3. You ran the code path you changed (test or example) and saw the output.

## Rust rules for this repo
- No `unwrap()` / `expect()` / indexing on input from outside the process (files, network,
  CLI, env, LLM output). Return `Result` with our error type; `?` with context.
- Never add `#[allow(...)]`, `unsafe`, or a new dependency without asking me first.
- Never ignore an error with `let _ =` or `.ok()` unless a comment says why it's safe.
- Prefer borrowing: take `&str` / `&[T]` when you only read. Don't `clone()` to silence the
  borrow checker; explain the ownership problem to me instead if you're stuck.
- No `Arc<Mutex<_>>` unless the data is actually shared across threads or tasks.
- Async: never block inside `async fn` (no `std::fs`, `std::thread::sleep`, heavy CPU);
  use `tokio::fs` or `spawn_blocking`.
- Errors: `thiserror` in libraries, `anyhow` only in binaries and tests.

## How to work
- For anything bigger than one function: propose a short plan (files, signatures, tests) and
  wait for my OK before editing.
- Write or update the tests first, show them failing, then implement.
- Keep diffs small and focused; don't reformat or refactor unrelated code.
- If the compiler fights you for more than two attempts, stop and explain the error instead of
  restructuring the design on your own.

## Gotchas
<!-- Add a line every time the agent makes the same mistake twice. This section is where
     CLAUDE.md earns its keep. -->
-
