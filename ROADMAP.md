# Roadmap

Planned at **~7 hours per week** (about one hour a day). Total: ~266 hours, ~39 weeks. Faster routes are in the README.

| Weeks | Part | Phase | Lessons | Effort | Goal |
|---|---|---|---|---|---|
| 1–1 | Part 1 | Phase 00: Setup & Tooling | 6 | ~6 h | A fast feedback loop is the first thing you build. |
| 2–3 | Part 1 | Phase 01: Core Language | 9 | ~14 h | Types are documentation the compiler checks. |
| 4–5 | Part 1 | Phase 02: Ownership & Memory | 8 | ~16 h | Every value has exactly one owner. Everything else follows. |
| 6–7 | Part 1 | Phase 03: Traits, Generics & Errors | 8 | ~16 h | Traits are protocols the compiler enforces. |
| 8–10 | Part 2 | Phase 04: The Build Loop: Developing with a Coding Agent | 12 | ~20 h | Your idea, the agent's code, your review. Repeat until it's right. |
| 11–12 | Part 2 | Phase 05: Data, Serialization & CLIs | 7 | ~14 h | AI is mostly moving data around. Do it fast and correctly. |
| 13–14 | Part 2 | Phase 06: Concurrency & Performance | 7 | ~14 h | No GIL. No excuses. No data races. |
| 15–16 | Part 2 | Phase 07: Async Rust | 7 | ~16 h | Agents spend most of their life waiting on I/O. |
| 17–18 | Part 2 | Phase 08: Web Services | 6 | ~12 h | Every agent needs an API surface. |
| 19–21 | Part 2 | Phase 09: LLM Engineering in Rust | 8 | ~18 h | The model is an API. Everything around it is software. |
| 22–24 | Part 2 | Phase 10: Agent Engineering | 9 | ~20 h | An agent is a loop. Build the loop yourself. |
| 25–26 | Part 2 | Phase 11: Tools & Protocols (MCP) | 8 | ~16 h | Tools are how agents touch the world. Make them safe and fast. |
| 27–28 | Part 2 | Phase 12: Python Interop (PyO3) | 7 | ~14 h | Keep Python for orchestration. Move the hot path to Rust. |
| 29–31 | Part 2 | Phase 13: ML & Inference in Rust | 7 | ~18 h | Understand the tensor, then trust the framework. |
| 32–33 | Part 2 | Phase 14: Production Rust | 6 | ~12 h | If it compiles, it's a start. If it's observed, it's production. |
| 34–39 | Part 2 | Phase 15: Capstone Projects | 5 | ~40 h | Ship something other people use. |

## Milestones

- **End of Part 1 (Phase 03):** you can read most Rust code and explain ownership, borrowing, traits and errors in your own words.
- **End of Phase 04:** you run the build loop out of habit: spec first, small slices, real review.
- **End of Phase 07:** you've built async services that call LLM APIs.
- **End of Phase 11:** you've shipped your own agent loop and an MCP server in Rust.
- **End of Phase 12:** you've shipped a Rust-accelerated Python package.
- **End of Phase 15:** two or more capstones you can show, with the loop logs to prove you directed them.
