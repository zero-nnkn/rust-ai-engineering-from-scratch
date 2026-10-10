---
name: rust-review
description: Review a Rust diff written by a coding agent (or a human) for problems the compiler, clippy and the existing tests cannot catch. Use when asked to review Rust changes or before merging agent-written Rust.
---

# Rust review

Assume the code already compiles, passes `cargo clippy -- -D warnings` and passes its tests.
Your job is everything those checks miss. Review the diff (`git diff` against the base branch)
in this order and report findings as a table: file:line · problem · consequence · suggested fix ·
severity (blocker / should-fix / nit).

1. **Boundaries.** New or changed `pub` signatures, `#![allow]`/`#[allow]` (treat every new one
   as a blocker unless justified in a comment that names the lint and the reason), `unsafe`
   (require a real invariant, not "never empty"), new dependencies (why, maintained, features).
2. **Untrusted input.** Data from files, network, env, CLI args, LLM output or tool arguments
   reaching `unwrap`, `expect`, slice indexing, `as` casts, `parse().unwrap()`, unbounded
   loops or allocations. Must be validated and return `Result`.
3. **Errors.** `let _ =`, `.ok()`, `unwrap_or_default()`, empty `Err(_) => {}` arms. Was the
   error handled, propagated with context, or hidden? Hidden is a blocker.
4. **Design.** Owned params that are only read (`String` → `&str`, `Vec<T>` → `&[T]`),
   `clone()` used to escape the borrow checker, `Arc<Mutex<_>>` without concurrency,
   O(n²) membership checks, blocking calls inside `async fn`.
5. **Tests, adversarially.** For each changed behaviour: what wrong implementation would still
   pass these tests? Flag tests without assertions, happy-path-only suites, and missing tests
   for empty input, malformed input and boundary values. Propose the missing tests concretely.

Rules:
- Do not rewrite the code in the review. Point at lines and explain.
- Don't report style issues that `rustfmt` or default clippy already enforce.
- End with a verdict: **merge**, **merge after should-fixes**, or **blocked**, and the
  three most important findings.
