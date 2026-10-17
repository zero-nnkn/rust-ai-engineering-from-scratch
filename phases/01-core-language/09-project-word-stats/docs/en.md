# Project: token & word statistics CLI

> **Motto:** Small tool, real edges: bad input, missing files, non-English text, exit codes.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 01 · Core Language | 09 | Build | ~3 h | `word-stats` CLI |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

Before you send a corpus to a model or an embedding API, you want to know what's in it: how
many tokens it will cost, how many unique words, what dominates the vocabulary. You've probably
written this in Python as a 20-line script that crashes on an empty file and miscounts
non-English text.

This project builds it properly in Rust, using every idea from Phase 01:

| Idea | Where it's used |
|---|---|
| structs and `impl` (01-03) | `Config`, `Stats`, `Display for Stats` |
| enums and `match` (01-04) | `Input::{File, Stdin}`, `UsageError`, argument dispatch |
| `Option`/`Result` and `?` (01-05) | parsing flags, reading files, `longest_word` |
| collections (01-06) | `HashMap` word counts, sorted top-N |
| strings and UTF-8 (01-07) | chars vs bytes, normalising words |
| modules and crates (01-08) | `lib.rs` + `args.rs` + `stats.rs`, thin `main.rs` |

## Concept

### The shape of a good CLI

```
main.rs   parse args → read input → compute → print → exit code      (I/O only)
args.rs   Vec<String> → Result<Config, UsageError>                    (pure, tested)
stats.rs  &str → Stats                                                (pure, tested)
```

Everything that can be tested without touching the terminal or the file system lives in the
library. `main.rs` is the only place that does I/O, and it stays short enough to read in one go.

### Exit codes and stderr

| Situation | Output | Exit code |
|---|---|---|
| success | stats on **stdout** | 0 |
| `--help` | usage on stdout | 0 |
| bad arguments | message on **stderr** | 2 (usage error convention) |
| file can't be read | message on stderr | 1 |

`main` returns `ExitCode` instead of calling `std::process::exit`, so destructors still run.
Errors go to stderr so they don't end up in a file when someone writes `word-stats a.txt > out.txt`.

### A first look at traits

`stats.rs` defines `trait TextStats` and implements it for `str`, so you can call
`text.stats(5, 4.0)` on any string. It's a preview of Phase 03, where traits get a full lesson.

### Token estimates

`est. tokens` is characters ÷ `--chars-per-token` (4.0 by default, roughly right for English
with common tokenizers, often lower for other languages). It's an estimate for budgeting. Phase 09
builds a real tokenizer.

## Explore

```bash
cargo run -p project-word-stats -- phases/01-core-language/09-project-word-stats/code/samples/ownership.txt --top 3
echo "Xin chào thế giới, xin chào!" | cargo run -p project-word-stats -- --top 2
cargo run -p project-word-stats -- --top zero;  echo "exit=$?"
cargo run -p project-word-stats -- missing.txt; echo "exit=$?"
cargo test -p project-word-stats
```

Expected output of the first command:

```
bytes              445
characters         445
lines                8
words               79
unique words        56
est. tokens        112
avg word len      4.49
longest word  implications
top words (stopwords excluded):
     3  owner
     3  ownership
     3  value
```

The Vietnamese line shows 35 bytes but 29 characters. The two error commands print to stderr
and exit with 2 and 1.

**Break it on purpose.**

1. In `stats.rs`, remove `.then_with(|| a.0.cmp(&b.0))` from the sort. Run the sample a few times.
   Does the order of `owner`, `ownership` and `value` change between runs? Why?
2. In `main.rs`, replace the `match read_input(...)` with `read_input(&config.input).unwrap()`
   and run it on `missing.txt`. Compare that output with the friendly error.
3. Run `cargo run -p project-word-stats < /dev/null`. Which fields would divide by zero if
   `compute` didn't check for empty input?

## Practice

1. Add `--lowercase false` to keep the original case. Which module changes, and does `main.rs` need to?
2. Add `--json` to print the stats as JSON, built by hand with `format!` (escape `"` and `\` in
   words). Phase 05 replaces this with `serde`.
3. Add a "sentences" count. Decide what counts as a sentence end, write tests for the tricky
   cases (`"e.g."`, `"..."`, `"3.14"`), and document your choice.
4. Accept several files (`word-stats a.txt b.txt`) and print totals plus per-file lines. Which
   `UsageError` variant goes away, and which new error cases appear?
5. Time it on a large file: `cargo build --release -p project-word-stats`, then run
   `target/release/word-stats` on a few hundred MB of text. Compare with your Python version.

## Ship It

The `word-stats` binary (named by the `[[bin]]` section in `Cargo.toml`). Install it on your machine:

```bash
cargo install --path phases/01-core-language/09-project-word-stats/code
word-stats --help
```

## Check Your Understanding

1. Why does `parse_args` take `IntoIterator<Item = String>` instead of reading `std::env::args()` itself?
2. Why do errors go to stderr, and why does a usage error exit with 2 instead of 1?
3. Why is the top-words sort given a tie-breaker?
4. Which parts of this program would you trust an AI agent to write unsupervised, and which would you review line by line? (You'll revisit this answer in Part 2.)

## Further Reading

- *The Rust Programming Language*, chapter 12 "An I/O Project: Building a Command Line Program"
- *Command Line Applications in Rust* (the CLI book): "Nice errors" and "Exit codes"
