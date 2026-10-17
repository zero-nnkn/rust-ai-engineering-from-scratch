# Cargo cheat sheet

## Everyday loop
| Command | Use |
|---|---|
| `cargo check` | fastest type-check; run on every save (or use `bacon` / `cargo watch`) |
| `cargo clippy --all-targets -- -D warnings` | lint; treat warnings as errors |
| `cargo fmt` | format |
| `cargo test` | unit + integration + doc tests |
| `cargo test <name>` | only tests whose name contains `<name>` |
| `cargo test -- --nocapture` | show `println!` output from tests |
| `cargo run -- <args>` | run; args after `--` go to your program |

## Selecting what to build (workspaces)
| Flag | Meaning |
|---|---|
| `-p <package>` | one package of the workspace |
| `--workspace` | every package |
| `--bin <name>` | one binary (`src/main.rs` or `src/bin/<name>.rs`) |
| `--example <name>` | `examples/<name>.rs` |
| `--lib` / `--tests` / `--all-targets` | which targets |
| `--release` | optimized build (`target/release/`); always use for timing |

## Dependencies
| Command | Use |
|---|---|
| `cargo add serde --features derive` | add a dependency |
| `cargo add --dev pretty_assertions` | test-only dependency |
| `cargo remove serde` | remove |
| `cargo tree -i <crate>` | who depends on `<crate>`? |
| `cargo update -p <crate>` | update one crate in `Cargo.lock` |

## Docs and inspection
| Command | Use |
|---|---|
| `cargo doc --open` | docs for your crate and all dependencies, offline |
| `cargo metadata --format-version 1` | the workspace as JSON (for scripts) |
| `cargo clean -p <package>` | remove build output for one package |

## Python → Cargo
| Python | Cargo |
|---|---|
| `pyproject.toml` | `Cargo.toml` |
| `uv.lock` / `poetry.lock` | `Cargo.lock` |
| `uv add x` | `cargo add x` |
| `pytest` | `cargo test` |
| `python -m pkg` | `cargo run` |
| `python -m build` + `twine upload` | `cargo publish` |
