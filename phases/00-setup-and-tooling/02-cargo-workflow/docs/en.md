# Cargo: build, run, test, workspaces

> **Motto:** One tool for the whole loop: Cargo builds, runs, tests, documents and publishes.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 00 · Setup & Tooling | 02 | Build | ~1 h | `outputs/cargo-cheatsheet.md` |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

A Python project has a tool for every job: `uv`/`pip` for dependencies, `pyproject.toml`
for metadata, `pytest` for tests, `python -m` to run, `build`/`twine` to publish, and
something else again for a monorepo. Each has its own config and its own idea of where
the project root is.

In Rust, **Cargo** does all of that, with one manifest (`Cargo.toml`) and one lockfile
(`Cargo.lock`). Learning its handful of commands well pays off in every lesson after this one.

## Concept

**Package, crate, target, workspace.** These four words come up constantly.

| Term | What it is | Python analogy |
|---|---|---|
| **package** | a folder with a `Cargo.toml` | a project with a `pyproject.toml` |
| **crate** | one unit of compilation: a library or a binary | a module tree you can import, or a script |
| **target** | what gets built from a package: `lib`, each binary, tests, examples, benches | the package plus its entry points |
| **workspace** | several packages sharing one `Cargo.lock` and one `target/` folder | a monorepo with a shared lockfile |

A package has **at most one library** (`src/lib.rs`) and **any number of binaries**
(`src/main.rs`, plus `src/bin/*.rs`). Keep the logic in the library and keep `main.rs` thin.
Tests can then exercise the library directly, and other crates can reuse it.

```
cargo-workflow/                 ← package (Cargo.toml)
├── src/lib.rs                  ← library crate "cargo_workflow" (dashes become underscores)
└── src/main.rs                 ← binary crate, uses cargo_workflow::slugify
```

This repository is itself a workspace. The root `Cargo.toml` says
`members = ["phases/*/*/code"]`, so every lesson's `code/` folder is a package, and
`-p <name>` picks one. One workspace gotcha: build settings like `[profile.release]` only
work in the **root** `Cargo.toml`. Put one in a member package and Cargo warns that it's
ignored.

**The commands you'll actually use:**

| Command | What it does | When |
|---|---|---|
| `cargo check` | type-checks, no binary produced | constantly: it's the fastest feedback |
| `cargo build` | compiles a debug binary into `target/debug/` | when you need the binary |
| `cargo run -- <args>` | build + run; args after `--` go to your program | trying things |
| `cargo test` | unit, integration and doc tests | after every change |
| `cargo build --release` | optimized binary in `target/release/` | benchmarks, shipping |
| `cargo doc --open` | HTML docs for your code and every dependency | reading APIs |
| `cargo add <crate>` / `cargo remove` | edit `[dependencies]` for you | adding a crate |
| `cargo tree` | the dependency graph | "why is this crate here?" |

Where the analogy breaks: **debug builds are slow, release builds are fast.** The gap is
often 10–50×. Python has nothing like it. Never benchmark a debug build.

## Explore

```bash
cargo run -p cargo-workflow
cargo run -p cargo-workflow -- "Agents, MCP & You"
cargo test -p cargo-workflow
```

Expected output of the second command:

```
title: Agents, MCP & You
words: 4
slug:  agents-mcp-you
built by cargo-workflow v0.1.0
```

`cargo test` runs **three** groups: the unit tests in `lib.rs` (3), the binary's tests (0),
and the **doc test**: the example in the `///` comment above `slugify` is compiled and run.
Documentation examples that can't go stale are one of Rust's nicest habits.

Now look around:

1. `ls target/debug/`: the binary `cargo-workflow` is there. Run it directly.
2. `cargo build --release -p cargo-workflow` and compare the time with `cargo build`.
3. `cargo doc -p cargo-workflow --open`: your `///` comments became a website.

**Break it on purpose.** In `main.rs`, change `use cargo_workflow::{slugify, word_count};`
to `use cargo-workflow::slugify;`. Read the error: crate names use underscores in code,
even when the package name has dashes.

## Practice

1. Add a second binary: create `src/bin/count.rs` with a `main` that prints `word_count` for
   its arguments. Run it with `cargo run -p cargo-workflow --bin count -- a b c`. Why does
   `cargo run -p cargo-workflow` now complain, and how does `default-run` in `Cargo.toml` fix it?
2. Add an `examples/demo.rs` that calls `slugify` on three titles, and run it with
   `cargo run -p cargo-workflow --example demo`. When would you use an example instead of a binary?
3. Run `cargo add --dry-run --package cargo-workflow serde` (dry run: nothing is changed) and
   read what it would write. Then open the root `Cargo.lock`: why is there one lockfile for
   the whole workspace instead of one per lesson?
4. Run `cargo check -p cargo-workflow` and `cargo build -p cargo-workflow` back to back and
   time both. Which one belongs in your edit loop?

## Ship It

`outputs/cargo-cheatsheet.md`: the commands above plus the workspace flags (`-p`,
`--workspace`, `--bin`, `--example`, `--release`). Add to it as you learn more.

## Check Your Understanding

1. What's the difference between a package and a crate? How many library crates can a package have?
2. Why put the logic in `lib.rs` instead of `main.rs`, even for a small CLI?
3. Why does a workspace share one `target/` folder and one `Cargo.lock`?
4. Why should you never measure performance with `cargo run` without `--release`?

## Further Reading

- *The Cargo Book*: "Cargo Guide" and "Workspaces"
- *The Rust Programming Language*, chapter 7: "Managing Growing Projects with Packages, Crates, and Modules"
