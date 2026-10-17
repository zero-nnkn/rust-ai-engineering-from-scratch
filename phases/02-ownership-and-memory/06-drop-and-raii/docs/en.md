# Drop & RAII: resources that clean themselves up

> **Motto:** Tie every resource to an owner. When the owner goes, the resource goes.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 02 · Ownership & Memory | 06 | Build | ~1.5 h | — |

**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.

## Problem

An agent runtime opens a lot of things that must be closed: files, temp directories for tools,
locks, network connections, tracing spans, GPU buffers. Python offers three ways to close them,
and each has a gap:

- **`try/finally`**: correct but easy to forget, and verbose.
- **`with` blocks**: good, but only if the caller remembers to use `with`. Nothing stops
  `f = open(...)` without it.
- **`__del__`**: runs "at some point", maybe never, maybe during interpreter shutdown when half
  the modules are gone.

Rust has one mechanism, **RAII** (Resource Acquisition Is Initialization): a value that owns a
resource releases it in its `Drop` implementation, and Drop runs **deterministically** when the
owner goes out of scope, on every exit path, including `?`, early `return`, and panics.

## Concept

### `Drop` runs at a known point

```rust
impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn run_tool(input: &str) -> std::io::Result<usize> {
    let ws = TempWorkspace::create("tool")?;
    fs::write(ws.path().join("in.txt"), input)?;   // if this fails, ws is dropped → cleaned up
    ...
    Ok(n)                                          // and here → cleaned up
}
```

There's no `with`, no `finally`, nothing the caller can forget. Ownership (Lesson 01) decides
*when*: a value is dropped when its owner goes out of scope, or when it's moved somewhere that
drops it.

| Python | Rust |
|---|---|
| `with open(p) as f:` | `let f = File::open(p)?;` (closed when `f` goes out of scope) |
| `with lock:` | `let guard = mutex.lock()?;` (unlocked when `guard` drops) |
| `try/finally: cleanup()` | a guard type with `impl Drop` |
| `del x` (maybe frees) | `drop(x)` (frees now) |
| `__del__` | `Drop`, but deterministic |

### Drop order

- Local variables: **reverse** order of declaration (last created, first dropped), like nested
  `with` blocks.
- Struct fields and `Vec` elements: in order (first field/element first).
- `drop(x)` ends a value early. It's just a function that takes ownership and does nothing.
- A moved-out value is not dropped twice: only its final owner drops it.

### Guards you'll use constantly

`MutexGuard` (unlocks), `File` (closes), `TcpStream` (disconnects), `JoinHandle`, and in
async code, `tokio` tasks and timeouts. Writing your own guard is the idiomatic way to make
"must always happen afterwards" impossible to forget: tracing spans, metrics timers, temp files,
"restore the previous config".

### Limits

- `Drop` can't return an error. Cleanup that can fail meaningfully (flushing a write you care
  about) deserves an explicit method like `fn finish(self) -> Result<...>`, with `Drop` as the
  fallback.
- `std::mem::forget` and reference cycles with `Rc` (Lesson 05) can stop `Drop` from running.
  That's *safe* (memory-safe) but leaks. Don't rely on `Drop` for correctness of unsafe code.
- A panic during a panic aborts the process, so don't panic inside `drop`.

## Explore

```bash
cargo run -p drop-and-raii
cargo test -p drop-and-raii
```

Expected output:

```
trace   : enter agent_step
trace   : enter llm_call
trace   : exit  llm_call
trace   : enter tool_call
trace   : exit  tool_call
trace   : exit  agent_step
trace   : enter agent_step
trace   : enter llm_call
trace   : exit  llm_call
trace   : enter tool_call
trace   : exit  tool_call
trace   : exit  agent_step
tool ok : 10 bytes, workspace still exists: false
tool err: tool rejected the input
drops   : v0 → v1 → c → b → a
```

The second `agent_step` returns an error early, and its trace is identical: both open spans still
close. The last line shows drop order: `v` was dropped explicitly first (elements in order), then
the locals in reverse.

**Break it on purpose.**

1. Remove `impl Drop for Span` entirely and run the tests. Which fail? Every "exit" line is now
   gone, and nothing else in the code had to remember to write them.
2. In `agent_step`, change `let _step = Span::enter(...)` to `let _ = Span::enter(...)`. Run
   `cargo test -p drop-and-raii spans`. Read the new trace: `_` doesn't bind the value, so it's
   dropped **immediately**. This is a common bug with lock guards too.
3. Add `std::mem::forget(ws);` at the end of `run_tool` before `Ok(...)` (you'll need to clone
   the path first). Run the test and check your temp directory: the cleanup never runs.

## Practice

1. Write a `Timer` guard that records the elapsed time of a block into a
   `RefCell<Vec<(String, Duration)>>` when dropped. Use it around three steps and print the slowest.
   (Use `std::time::Instant`.)
2. Explain the difference between `let _ = guard()`, `let _g = guard()` and `let g = guard()`
   in terms of when the guard drops. Write a test that shows it.
3. Write a `ConfigOverride` guard: on creation it sets a value in a `RefCell<HashMap<..>>` and
   remembers the old value; on drop it restores it. Test it with an early `return` and with a panic
   (`catch_unwind`, as in the test file).
4. `TempWorkspace::drop` ignores a deletion failure except for a warning. Add
   `fn close(self) -> std::io::Result<()>` that reports the error properly. How do you stop
   `Drop` from trying again after `close` has run? (Hint: `Option<PathBuf>` and `take()`.)

## Check Your Understanding

1. When exactly does `Drop` run? Name three exit paths where it still runs.
2. Why do local variables drop in reverse order of declaration?
3. Why is `let _ = mutex.lock()` a bug, and what should it be?
4. Why can't `Drop` return a `Result`, and what pattern handles cleanup that can fail?

## Further Reading

- *The Rust Programming Language*, chapter 15.3 "Running Code on Cleanup with the Drop Trait"
- *The Rust Reference*: "Destructors" (exact drop order rules)
