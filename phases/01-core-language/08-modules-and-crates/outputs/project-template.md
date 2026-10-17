# Rust library + CLI project template

```
my-tool/
├── Cargo.toml
├── src/
│   ├── lib.rs          # crate root: `pub mod` declarations + `pub use` re-exports = the public API
│   ├── error.rs        # the crate's error enum (Phase 03 · Lesson 06)
│   ├── <domain>.rs     # one module per concept (index, chunker, client…)
│   ├── <domain>/       # child modules when a concept grows
│   │   └── <part>.rs
│   └── main.rs         # thin CLI: parse args → call the library → print; no logic here
├── tests/
│   └── <feature>.rs    # integration tests: public API only
└── examples/
    └── basic.rs        # runnable usage example (cargo run --example basic)
```

## Rules
1. **Private by default.** Start every item private; make it `pub` only when another crate needs it.
2. **`pub(crate)`** for things shared between your own modules but not part of the API.
3. **Re-export the API from `lib.rs`** (`pub use index::{Index, SearchHit};`) so you can move
   files without breaking users.
4. **Struct fields private**, with methods that keep invariants (e.g. sorted, within budget).
5. **`main.rs` stays thin.** If a function in `main.rs` deserves a test, move it to the library.
6. **One concept per module.** If a module needs a long comment explaining its sections,
   split it.
7. **Unit tests next to the code** (`#[cfg(test)] mod tests`), behaviour tests in `tests/`.

## New project
```bash
cargo new my-tool --lib            # library
touch my-tool/src/main.rs          # add a binary to the same package
cargo add --package my-tool <deps> # as needed
```
