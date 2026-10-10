# Hooks: Automatic fmt, clippy & Tests After Every Agent Edit

> **Motto:** Don't ask the agent to remember to check. Make checking automatic.

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| 04 · The Build Loop | 03 | Build | ~1.5 h | `outputs/` hook scripts + `settings.json` |

**Part 2 · Build with a coding agent.** Requires Lesson 02. The setup here is for Claude Code; most coding agents
have an equivalent (hooks, rules, or a post-edit command).

## Problem

`CLAUDE.md` says "run clippy and tests before saying done." Most of the time the agent does.
Sometimes it doesn't, especially late in a long session. And when it writes five files before
compiling once, it then spends many turns untangling errors that one early `cargo check`
would have caught at the first file.

Instructions are requests. **Hooks are guarantees**: shell commands the agent runtime executes
at fixed points, whether or not the model remembers.

## Concept

```
agent edits src/foo.rs
        │
        ▼
PostToolUse hook ──► rustfmt foo.rs ──► cargo clippy -D warnings (that crate)
        │                                   │
        │                         fails ────┴──► exit 2: errors go back to the agent
        ▼                                         → it fixes them before writing more
agent says "done"
        │
        ▼
Stop hook ──► Rust files changed? ──► cargo test --workspace
                                          │
                                fails ────┴──► exit 2: agent keeps working
                                               (at most twice in a row, then it must
                                                stop and report the failure to you)
```

In Claude Code, hooks are configured in `.claude/settings.json`. Each hook receives the
event as JSON on stdin (for edits, the path is in `tool_input.file_path`).
`CLAUDE_PROJECT_DIR` points at the repo root. For `PostToolUse`, exit code 2 shows your
stderr to the agent; for `Stop`, exit code 2 keeps it working. Check the current hooks
documentation at <https://code.claude.com/docs/en/hooks>, since details evolve.

Why Rust benefits more than most languages: the compiler and clippy give **precise,
machine-readable feedback at every step**. A Python agent learns about a type error at runtime,
if a test happens to cover it. A Rust agent with this hook learns about it seconds after the
edit, with the exact line and often the fix. That's why the build loop (Lesson 01) can keep slices
small: the compiler checks each one before you review it.

## Build It

The files in `outputs/` are ready to install. They also run in this repository (see
`.claude/` at the root).

```bash
# in your practice repo from Lesson 02
mkdir -p .claude/hooks
cp <this-lesson>/outputs/*.sh .claude/hooks/ && chmod +x .claude/hooks/*.sh
cp <this-lesson>/outputs/settings.json .claude/settings.json
```

Test the hooks **without** an agent first, by feeding them the JSON they'd receive:

```bash
# should exit 0 (clean file)
echo '{"tool_input":{"file_path":"'$PWD'/src/lib.rs"}}' | .claude/hooks/post-edit-check.sh; echo "exit=$?"

# now add `let x = 5;` (unused) inside a function in src/lib.rs and rerun: exit=2 + the clippy error
```

Then start an agent session and ask for a small change that will initially fail clippy
(e.g. "add a function that sums a slice using an index loop"). Watch the hook's feedback
arrive and the agent fix it without you saying anything.

Read both scripts. They're short and they're yours to change: the loop guard, which commands
run, how much output is sent back.

## Use It

1. **Speed.** Clippy on every edit is slow in a big workspace. Change the post-edit hook to
   run `cargo check` per edit and leave clippy for the Stop hook. Measure the difference.
2. **Block dangerous commands.** Write a `PreToolUse` hook (matcher `Bash`) that refuses
   `cargo publish` and `git push --force` with exit code 2 and a clear message.
3. **Gate `#[allow]`.** Extend the post-edit hook: if the diff adds `#[allow(` or `unsafe`,
   return exit 2 with "ask the user first". This enforces Lesson 02's rule mechanically.
4. **CI parity.** Make sure the hooks run the same commands as CI. If they differ, what goes wrong?

## Ship It

- `outputs/post-edit-check.sh`: format and lint the edited crate, feeding errors back.
- `outputs/stop-run-tests.sh`: no "done" while tests fail, with a loop guard.
- `outputs/settings.json`: wires both into Claude Code.

## Check Your Understanding

1. Why does the Stop hook need a loop guard, and what should the agent do once it's exhausted?
2. Why run fast checks per edit and slow checks at the end, instead of everything every time?
3. What can hooks guarantee that `CLAUDE.md` can't, and what can `CLAUDE.md` do that hooks can't?
4. A hook prints 2,000 lines of compiler output. Why is that worse than 40 lines?
