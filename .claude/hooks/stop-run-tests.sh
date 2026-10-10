#!/usr/bin/env bash
# Claude Code Stop hook
# Before the agent says "done": if Rust code changed, the workspace tests must pass.
# Exit 2 sends the failures back and the agent keeps working.
# Loop guard: blocks at most MAX_BLOCKS times in a row, then lets the agent stop
# (so it can explain the failure to you instead of thrashing).
set -uo pipefail
MAX_BLOCKS=2

input=$(cat)
# Some Claude Code versions send stop_hook_active=true when already continuing
# because of a Stop hook; honour it if present.
active=$(printf '%s' "$input" | python3 -c \
  'import json,sys; print(json.load(sys.stdin).get("stop_hook_active", False))' 2>/dev/null)
[[ "$active" == "True" ]] && exit 0

root="${CLAUDE_PROJECT_DIR:-$(pwd)}"
cd "$root" || exit 0
[[ -f Cargo.toml ]] || exit 0

# Skip when no Rust sources or manifests changed (git repos only).
if git rev-parse --git-dir >/dev/null 2>&1; then
  [[ -n "$(git status --porcelain -- '*.rs' '*Cargo.toml' 2>/dev/null)" ]] || exit 0
fi

counter="${TMPDIR:-/tmp}/claude-stop-hook-$(printf '%s' "$root" | cksum | cut -d' ' -f1)"
blocks=$(cat "$counter" 2>/dev/null || echo 0)

if out=$(cargo test --workspace --quiet 2>&1); then
  rm -f "$counter"
  exit 0
fi

if (( blocks >= MAX_BLOCKS )); then
  rm -f "$counter"
  exit 0   # give up blocking; the agent should now report the failure honestly
fi
echo $((blocks + 1)) > "$counter"
{
  echo "Tests are failing. Don't stop yet: fix them, or explain to the user exactly why they fail."
  echo "$out" | grep -E 'FAILED|panicked|error(\[|:)|test result' | head -30
} >&2
exit 2
