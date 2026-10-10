# rustup, cargo & rust-analyzer

> **Motto:** A fast feedback loop is the first thing you build.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 00 · Setup & Tooling | 01 | Build | ~1 h | `verify-env` output (your first evidence) |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

In Python you juggle `pyenv`, `venv`, `pip`/`uv`/`poetry`, `pytest`, `ruff` and `mypy`.
Rust ships one toolchain manager (**rustup**) and one build tool (**cargo**) that covers
building, dependencies, testing, benchmarking, docs and publishing. Set it up once, properly.

## Concept

| Python | Rust | Notes |
|---|---|---|
| `pyenv` | `rustup` | toolchain versions & components |
| `pip` / `uv` + `pyproject.toml` | `cargo` + `Cargo.toml` | deps, build, run |
| `requirements.lock` / `uv.lock` | `Cargo.lock` | reproducible builds |
| `pytest` | `cargo test` | built in, no plugin needed |
| `ruff check` | `cargo clippy` | linter with hundreds of lints |
| `ruff format` / `black` | `cargo fmt` | formatter |
| `mypy` | the compiler | types are always checked |
| PyPI | crates.io | package registry |
| Pylance | rust-analyzer | editor language server |

## Explore

1. Install Rust: follow the official instructions at <https://rustup.rs>.
2. Add components: `rustup component add clippy rustfmt rust-analyzer`.
3. Install the **rust-analyzer** extension in your editor (VS Code, Zed, RustRover, Neovim).
   Turn on inlay hints: seeing inferred types inline is the fastest way to learn.
4. From the repository root, run the check:

```bash
cargo run -p verify-env
```

Expected (versions will differ):

```
Rust for AI Engineers · environment check

  ✓ rustc    rustc 1.xx.x (...)
  ✓ cargo    cargo 1.xx.x (...)
  ✓ clippy   clippy 0.1.xx (...)
  ✓ rustfmt  rustfmt 1.x.x-stable (...)
  ✓ git      git version 2.xx.x
  ✓ python3  Python 3.12.x
  · maturin  not found (optional, Phase 12 (Python packages))
  ...
All required tools found. Save this output as your first evidence.
```

## Practice

Read `code/src/main.rs`. Even before Phase 1 you can follow most of it. Notice:

- `const CHECKS: &[Check]` is a static table, like a module-level list of dataclasses.
- `probe()` returns `Option<String>`: either `Some(version)` or `None`. No exceptions.
- The `?` inside `probe()` means "if this is `None`/`Err`, return early".
- `main` returns `ExitCode`, so the script fails properly in CI.

Exercise: add a check for `cargo-watch` or `bacon` (both re-run `cargo check` on save),
install one, and use it for the rest of the course.

## Ship It

Save the terminal output to `LEARNING.md` under "Evidence". Every lesson from now on ends
with evidence: the command, its output, and what you changed.

## Check Your Understanding

1. What is the difference between `cargo build`, `cargo check` and `cargo run`? Which one should run on every save?
2. Why commit `Cargo.lock` for binaries?
3. What does `rustup update` change, and why is Rust's stability promise relevant to it?
