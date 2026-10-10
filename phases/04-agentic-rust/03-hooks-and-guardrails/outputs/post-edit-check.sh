#!/usr/bin/env bash
# Claude Code PostToolUse hook (matcher: Edit|Write)
# After the agent edits a .rs file: format it, then run clippy on its crate.
# On failure, exit 2: the errors go back to the agent as feedback, so it fixes
# them immediately instead of piling more code on top of a broken build.
set -uo pipefail

input=$(cat)
file=$(printf '%s' "$input" | python3 -c \
  'import json,sys; print(json.load(sys.stdin).get("tool_input",{}).get("file_path",""))' 2>/dev/null)

[[ "$file" == *.rs && -f "$file" ]] || exit 0

# Find the nearest Cargo.toml that defines a [package].
dir=$(cd "$(dirname "$file")" && pwd)
manifest=""
while [[ "$dir" != "/" ]]; do
  if [[ -f "$dir/Cargo.toml" ]] && grep -q '^\[package\]' "$dir/Cargo.toml"; then
    manifest="$dir/Cargo.toml"
    break
  fi
  dir=$(dirname "$dir")
done
[[ -n "$manifest" ]] || exit 0

edition=$(grep -m1 -E '^edition *= *"' "$manifest" | sed -E 's/.*"([0-9]+)".*/\1/')
rustfmt --edition "${edition:-2021}" "$file" 2>/dev/null || true

if ! out=$(cargo clippy --quiet --manifest-path "$manifest" --all-targets \
            --message-format short -- -D warnings 2>&1); then
  {
    echo "cargo clippy failed after editing $file. Fix these before writing more code:"
    echo "$out" | grep -v '^\s*$' | head -40
  } >&2
  exit 2
fi
exit 0
