"""Generate README, ROADMAP, phase READMEs and outline lessons from curriculum.json.

Usage (from the repository root):  python3 scripts/generate.py
Lesson docs that exist are never overwritten. A lesson counts as complete when its
docs/en.md no longer contains the "Outline lesson" note.
"""
import json
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent
C = json.loads((ROOT / "curriculum.json").read_text())
PHASES = C["phases"]
OUTLINE_MARKER = "📝 **Outline lesson.**"

PART_NAME = {1: "Part 1 · Learn Rust", 2: "Part 2 · Build with a coding agent"}
PART_LINE = {
    1: "**Part 1 · Learn Rust.** Understand the language, its principles and its mindset. Your AI agent is your tutor.",
    2: "**Part 2 · Build with a coding agent.** You bring the idea and the review; the agent writes the code; you loop until it's right.",
}


def phase_dir(p):
    return f"phases/{p['id']}-{p['slug']}"


def lesson_dir(p, i, slug):
    return f"{phase_dir(p)}/{i:02d}-{slug}"


def is_complete(p, i, slug):
    doc = ROOT / lesson_dir(p, i, slug) / "docs" / "en.md"
    return doc.exists() and OUTLINE_MARKER not in doc.read_text()


def package_name(p, i, slug):
    manifest = ROOT / lesson_dir(p, i, slug) / "code" / "Cargo.toml"
    if not manifest.exists():
        return None
    return manifest.read_text().split('name = "')[1].split('"')[0]


total_lessons = sum(len(p["lessons"]) for p in PHASES)
total_hours = sum(p["hours"] for p in PHASES)
complete = [(p, i, l) for p in PHASES for i, l in enumerate(p["lessons"], 1) if is_complete(p, i, l[0])]

# ------------------------------------------------------------------ outline lessons
HEADER = """# {title}

> **Motto:** {motto}

| Phase | Lesson | Type | Time | Ships |
|---|---|---|---|---|
| {pid} · {pname} | {num:02d} | {ltype} | ~{mins} min | {ships} |

{part_line}

> 📝 **Outline lesson.** Write it as you go, following [`LESSON_TEMPLATE.md`](../../../../LESSON_TEMPLATE.md),
> or draft it with your tutor (`/learn-rust draft {pid}-{num:02d}`). Delete this note when the lesson is written.
"""

BODY = {
    1: """
## Problem

<!-- What hurts in Python (or without this concept)? One concrete example. -->

## Concept

<!-- The principle in your own words. One diagram or table. The Python analogy, and where it breaks. -->

## Explore

<!-- A small example in code/. Run it, read it, change it, break it on purpose and read the
     compiler error. Ask your tutor about anything you can't explain line by line. -->

```bash
cargo run -p {package}
cargo test -p {package}
```

## Practice

<!-- 2–4 small exercises. Try first; ask the tutor for a hint before a solution;
     have it review what you wrote. -->

## Check Your Understanding

1.
2.
3.
""",
    2: """
## Problem

<!-- What are we building, and why does it matter in an AI system? -->

## Concept

<!-- The Rust principles and design choices you need in order to judge the agent's code.
     Learn these with your tutor before the loop starts. -->

## Your Idea → Spec

<!-- Your idea in 2–3 sentences, then the spec the agent must satisfy:
     public types/signatures, 2–4 tests (at least one failure case), constraints
     (dependencies allowed, no unsafe, error type…). -->

## The Loop

<!-- Run /build-loop with your spec. For each iteration record: what the agent did,
     what you found in review, what you asked it to change. Stop when the spec's tests pass
     AND you can explain every line you're keeping. -->

| # | Agent did | I found in review | I asked for |
|---|---|---|---|
| 1 | | | |

## Review Focus

<!-- The 3–5 things to look hardest at in this lesson's code
     (see Phase 04 · Lesson 05 for the general checklist). -->

## Ship It

<!-- The artifact you keep in outputs/: a CLI, crate, service, MCP server, prompt or skill. -->

## Check Your Understanding

1.
2.
3.
""",
}

for p in PHASES:
    for i, (slug, title, ltype, ships) in enumerate(p["lessons"], 1):
        d = ROOT / lesson_dir(p, i, slug)
        doc = d / "docs" / "en.md"
        (d / "outputs").mkdir(parents=True, exist_ok=True)
        if not doc.exists():
            doc.parent.mkdir(parents=True, exist_ok=True)
            mins = round(p["hours"] * 60 / len(p["lessons"]) / 15) * 15
            doc.write_text(
                HEADER.format(title=title, motto=p["motto"], pid=p["id"], pname=p["name"], num=i,
                              ltype=ltype, mins=mins, ships=ships, part_line=PART_LINE[p["part"]])
                + BODY[p["part"]].replace("{package}", slug)
            )
        keep = d / "outputs" / ".gitkeep"
        if not any(x for x in (d / "outputs").iterdir() if x.name != ".gitkeep"):
            keep.touch()
        elif keep.exists():
            keep.unlink()


# ------------------------------------------------------------------ tables
def lesson_table(p, link_prefix=""):
    rows = ["| # | Lesson | Type | Ships | Status |", "|---|---|---|---|---|"]
    for i, (slug, title, ltype, ships) in enumerate(p["lessons"], 1):
        status = "✅ Complete" if is_complete(p, i, slug) else "📝 Outline"
        rows.append(f"| {i:02d} | [{title}]({link_prefix}{i:02d}-{slug}/docs/en.md) | {ltype} | {ships} | {status} |")
    return "\n".join(rows)


# ------------------------------------------------------------------ phase READMEs
for idx, p in enumerate(PHASES):
    nav = []
    if idx > 0:
        q = PHASES[idx - 1]
        nav.append(f"← [Phase {q['id']}: {q['name']}](../{q['id']}-{q['slug']}/README.md)")
    nav.append("[All phases](../../README.md#contents)")
    if idx + 1 < len(PHASES):
        q = PHASES[idx + 1]
        nav.append(f"[Phase {q['id']}: {q['name']}](../{q['id']}-{q['slug']}/README.md) →")
    (ROOT / phase_dir(p) / "README.md").write_text(
        f"# Phase {p['id']}: {p['name']}\n\n> {p['motto']}\n\n"
        f"**{len(p['lessons'])} lessons · ~{p['hours']} hours** · {PART_LINE[p['part']]}\n\n"
        f"{lesson_table(p)}\n\n"
        f"Finish the phase with a quiz from your tutor (`/learn-rust quiz {p['id']}`), then move on.\n\n"
        + " · ".join(nav) + "\n"
    )

# ------------------------------------------------------------------ README
mermaid_nodes = "\n".join(f'  P{int(p["id"])}["Phase {p["id"]} — {p["name"]}"]' for p in PHASES)
mermaid = f"""```mermaid
flowchart TB
{mermaid_nodes}
  P0 --> P1 --> P2 --> P3 --> P4
  P4 --> P5 & P6 & P7
  P7 --> P8
  P5 --> P9
  P7 --> P9
  P9 --> P10 --> P11
  P6 --> P12 & P13
  P8 --> P14
  P11 & P12 & P13 & P14 --> P15
  classDef learn fill:#fde8d7,stroke:#c2410c,color:#1a1a1a
  classDef build fill:#dbeafe,stroke:#1d4ed8,color:#1a1a1a
  class P0,P1,P2,P3 learn
  class {",".join(f"P{int(p['id'])}" for p in PHASES if p["part"] == 2)} build
```

🟧 Part 1 · Learn Rust (with your tutor) · 🟦 Part 2 · Build with a coding agent (your loop)"""

loop_diagram = """```mermaid
flowchart LR
  I["💡 YOUR IDEA"] --> S["📐 SPEC<br/><sub>types + tests + constraints</sub>"]
  S --> A["🤖 AGENT CODES<br/><sub>small slice, hooks check it</sub>"]
  A --> R["🔍 YOU REVIEW<br/><sub>diff, tests, can I explain it?</sub>"]
  R -- "change this" --> A
  R -- "spec was wrong" --> S
  R -- "✅ accept" --> D["📦 SHIP"]
```"""

part1_shape = """```mermaid
flowchart LR
  M["MOTTO"] --> Pr["PROBLEM<br/><sub>what hurts in Python</sub>"]
  Pr --> C["CONCEPT<br/><sub>principle + Python analogy</sub>"]
  C --> E["EXPLORE<br/><sub>run · read · break the example</sub>"]
  E --> P["PRACTICE<br/><sub>small exercises, tutor hints</sub>"]
  P --> Q["CHECK<br/><sub>tutor quiz</sub>"]
```"""

part2_shape = """```mermaid
flowchart LR
  M["MOTTO"] --> Pr["PROBLEM<br/><sub>what we're building</sub>"]
  Pr --> C["CONCEPT<br/><sub>what you need to judge the code</sub>"]
  C --> S["IDEA → SPEC"]
  S --> L["THE LOOP<br/><sub>agent codes ⇄ you review</sub>"]
  L --> F["REVIEW FOCUS"]
  F --> Sh["SHIP IT"]
```"""


def first_phase_link(pid):
    p = next(x for x in PHASES if x["id"] == pid)
    return f"[Phase {pid}]({phase_dir(p)}/README.md)"


def part_contents(part):
    out = []
    for p in PHASES:
        if p["part"] != part:
            continue
        out.append(
            f"<details>\n<summary><b>Phase {p['id']} — {p['name']}</b> "
            f"<code>{len(p['lessons'])} lessons · ~{p['hours']}h</code></summary>\n\n"
            f"> {p['motto']}\n\n{lesson_table(p, phase_dir(p) + '/')}\n\n</details>"
        )
    return "\n\n".join(out)


def done_line(p, i, l):
    pkg = package_name(p, i, l[0])
    run = f" — `cargo run -p {pkg}`" if pkg else ""
    return f"- Phase {p['id']} · [{l[1]}]({lesson_dir(p, i, l[0])}/docs/en.md){run}"


part_hours = {k: sum(p["hours"] for p in PHASES if p["part"] == k) for k in (1, 2)}
part_lessons = {k: sum(len(p["lessons"]) for p in PHASES if p["part"] == k) for k in (1, 2)}
done_list = "\n".join(done_line(p, i, l) for (p, i, l) in complete)

readme = f"""# 🦀 {C['title']}

**{C['tagline']}**

A Rust curriculum for engineers who already ship AI in Python and want the systems layer of
the agent era: fast tool servers, agent runtimes, MCP servers, inference, and
Rust-accelerated Python libraries. Built for how code gets written now: **you understand the
language deeply, and a coding agent does most of the typing, under your control.**

`{len(PHASES)} phases` · `{total_lessons} lessons` · `~{total_hours} hours` · `MIT`

> Python stays your orchestration language. Rust becomes the layer underneath it:
> the part that has to be fast, correct, concurrent and safe to run unattended.

---

## How this course works: two parts

| | Part 1 · Learn Rust | Part 2 · Build with a coding agent |
|---|---|---|
| **Phases** | 00–03 · {part_lessons[1]} lessons · ~{part_hours[1]} h | 04–15 · {part_lessons[2]} lessons · ~{part_hours[2]} h |
| **Goal** | The language, its principles and its mindset: ownership, borrowing, types, traits, errors | Develop real things in Rust: CLIs, services, agents, MCP servers, PyO3 packages, inference |
| **Your agent is** | Your **tutor**: explains, answers "why", quizzes you, reviews your attempts | Your **developer**: writes the code from your spec |
| **You** | Read, run, break and play with examples until you can explain them | Bring the idea, write the spec, review every diff, decide what ships |

Part 2 is one loop, repeated at every scale, and **you control every arrow**:

{loop_diagram}

Rust is unusually good for this loop. The compiler and clippy check the agent's code seconds
after each edit, types make your spec precise, and risky shortcuts (`unwrap`, `clone`,
`unsafe`, `#[allow]`) show up in every diff. Part 1 is what lets you review that code with
confidence; Phase 04 turns the loop into a habit.

## Start here: choose what you want to build

Everyone does **Part 1 (Phases 00–03)** and **Phase 04 (the build loop)**. Then pick a route:

| Your goal | Route after Phase 04 |
|---|---|
| Everything, in order | Phases 05 → … → 15 |
| Agents and MCP servers in Rust | {first_phase_link('07')} → 09 → 10 → 11 → 15 |
| Speed up existing Python AI code | {first_phase_link('06')} → 12 → 15 |
| Run models locally / inference | {first_phase_link('06')} → 13 → 15 |
| Production backend for AI products | {first_phase_link('07')} → 08 → 14 → 15 |

## How to use each lesson

**Part 1 · Learn Rust**
1. **Read** the Problem and Concept. Ask your tutor until you could explain it to a Python colleague.
2. **Explore** the example in `code/`: `cargo run -p <package>`, change it, break it, read the compiler error.
3. **Practice** the small exercises. Try first; ask for a hint before a solution; have the tutor review.
4. **Check** with the tutor's quiz. Anything you miss goes to the review queue in `LEARNING.md`.

**Part 2 · Build with a coding agent**
1. **Learn the Concept** with your tutor: the Rust ideas you'll need to judge the code.
2. **Write your idea as a spec**: types, signatures, tests (one failure case at least), constraints.
3. **Run the loop** with `/build-loop`: the agent codes a small slice → hooks check it → you review → you decide.
4. **Ship it** when the tests pass *and* you can explain every line you're keeping. Log the loop in `LEARNING.md`.

## Clone it and produce your first evidence

```bash
git clone <your-fork-url> rust-ai-engineering-from-scratch
cd rust-ai-engineering-from-scratch
cargo run -p verify-env                           # Phase 00 · Lesson 01: checks your toolchain
cargo run -p ownership-and-move                   # Phase 02 · Lesson 01: the core Rust idea
cargo test -p reviewing-agent-rust -- --ignored   # Phase 04 · Lesson 05: agent code that's green but wrong
cargo run -p agent-loop                           # Phase 10 · Lesson 01: where you're heading
```

Fully written lessons so far:

{done_list}

The other lessons are **outlines** with a fixed title, place and artifact; you write them as
you go (see [How this repo grows](#how-this-repo-grows)).

## The shape of the curriculum

{mermaid}

## The shape of a lesson

```
phases/<NN>-<phase>/<NN>-<lesson>/
├── code/        a Cargo crate (workspace member): cargo run -p <name>
├── docs/
│   └── en.md    the lesson
└── outputs/     what this lesson ships (prompt, skill, CLI, crate, MCP server…)
```

**Part 1 lessons**

{part1_shape}

**Part 2 lessons**

{part2_shape}

## Your agent setup (already in this repo)

| File | What it does |
|---|---|
| [`CLAUDE.md`](CLAUDE.md) (+ `AGENTS.md` symlink) | Tells any coding agent its role: tutor in Part 1, developer under your control in Part 2 |
| [`/learn-rust`](.claude/skills/learn-rust/SKILL.md) | Your tutor: placement, lessons, hints, reviews, quizzes, compiler-error explanations |
| [`/build-loop`](.claude/skills/build-loop/SKILL.md) | Runs Part 2's loop: idea → spec (you approve) → small slice → checks → your review → repeat |
| [`.claude/settings.json`](.claude/settings.json) + [hooks](.claude/hooks/post-edit-check.sh) | After every `.rs` edit: rustfmt + clippy `-D warnings`, errors fed back to the agent. Before it stops: tests must pass |
| [`rust-review` skill](phases/04-agentic-rust/05-reviewing-agent-rust/outputs/skill-rust-review.md) | A second reviewer for agent-written diffs (never the only one) |
| [Rust `CLAUDE.md` template](phases/04-agentic-rust/02-agent-setup/outputs/CLAUDE.md) | Copy into your own Rust projects |

**Tutor commands**

| Command | What it does |
|---|---|
| `/learn-rust start` | Placement questions, then a personal route in `LEARNING.md` |
| `/learn-rust` | Teaches the next lesson: warm-up, concept, explore, practice, quiz |
| `/learn-rust explain` | Paste a compiler error; learn the rule behind it |
| `/learn-rust quiz <phase>` | Eight-question phase check with lessons to review |
| `/learn-rust draft <lesson>` | Drafts an outline lesson with you |

**Build commands**

| Command | What it does |
|---|---|
| `/build-loop <your idea>` | Turns the idea into a spec with you, then loops: agent codes a slice, you review |
| `/build-loop review` | Walks you through the current diff with the review checklist before you decide |

## Contents

### Part 1 · Learn Rust

{part_contents(1)}

### Part 2 · Build with a coding agent

{part_contents(2)}

## How this repo grows

This is a **learning repo**: you complete it as you go.

1. Pick the next 📝 lesson on your route.
2. Write `docs/en.md` from [`LESSON_TEMPLATE.md`](LESSON_TEMPLATE.md) (or `/learn-rust draft <lesson>`).
3. Put example or project code in `code/` as a Cargo crate; the workspace picks it up automatically.
4. Put what the lesson ships in `outputs/`.
5. Delete the "Outline lesson" note and run `python3 scripts/generate.py`: the lesson turns ✅.
6. Commit, with the evidence in `LEARNING.md`.

CI (`.github/workflows/ci.yml`) runs `fmt`, `clippy -D warnings` and `test` on every crate.

## Core resources

| Resource | Use it for |
|---|---|
| *The Rust Programming Language* ("the Book") | Part 1 primary text |
| Rustlings | Extra practice alongside Part 1 |
| *Rust by Example* | Quick syntax lookups |
| Your coding agent's docs (e.g. Claude Code: code.claude.com/docs) | Phase 04: hooks, skills, subagents, permissions |
| *Rust Atomics and Locks* (Mara Bos) | Phase 06 depth |
| The tokio tutorial | Phase 07 |
| *Zero To Production In Rust* (Luca Palmieri) | Phases 08 and 14 |
| The MCP specification (modelcontextprotocol.io) | Phase 11: check the current revision, it changes |
| PyO3 user guide | Phase 12 |
| candle and burn docs/examples | Phase 13 |

## License

MIT
"""
(ROOT / "README.md").write_text(readme)

# ------------------------------------------------------------------ ROADMAP
weeks, rows = 0, []
for p in PHASES:
    w = max(1, round(p["hours"] / 7))
    rows.append(f"| {weeks + 1}–{weeks + w} | {PART_NAME[p['part']].split(' · ')[0]} | Phase {p['id']}: {p['name']} "
                f"| {len(p['lessons'])} | ~{p['hours']} h | {p['motto']} |")
    weeks += w
(ROOT / "ROADMAP.md").write_text(
    "# Roadmap\n\nPlanned at **~7 hours per week** (about one hour a day). "
    f"Total: ~{total_hours} hours, ~{weeks} weeks. Faster routes are in the README.\n\n"
    "| Weeks | Part | Phase | Lessons | Effort | Goal |\n|---|---|---|---|---|---|\n"
    + "\n".join(rows)
    + "\n\n## Milestones\n\n"
    "- **End of Part 1 (Phase 03):** you can read most Rust code and explain ownership, borrowing, traits and errors in your own words.\n"
    "- **End of Phase 04:** you run the build loop out of habit: spec first, small slices, real review.\n"
    "- **End of Phase 07:** you've built async services that call LLM APIs.\n"
    "- **End of Phase 11:** you've shipped your own agent loop and an MCP server in Rust.\n"
    "- **End of Phase 12:** you've shipped a Rust-accelerated Python package.\n"
    "- **End of Phase 15:** two or more capstones you can show, with the loop logs to prove you directed them.\n"
)
print(f"{len(PHASES)} phases, {total_lessons} lessons, {total_hours} h, {len(complete)} complete, ~{weeks} weeks")
