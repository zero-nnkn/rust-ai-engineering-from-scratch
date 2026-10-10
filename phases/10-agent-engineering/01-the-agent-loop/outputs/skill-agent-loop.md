---
name: rust-agent-loop
description: Scaffold a minimal, testable ReAct-style agent loop in Rust with a swappable LLM trait, a tool registry, error-as-data tool results and a step limit.
---

# Rust agent loop

When asked to add an agent loop to a Rust project:

1. Define `enum Message { User, Assistant { text, tool_calls }, ToolResult { call_id, content, is_error } }`.
2. Define `trait Llm { fn complete(&mut self, history: &[Message], tools: &[ToolSpec]) -> Message; }`
   so the model is swappable and tests can use a scripted implementation.
3. Define `trait Tool { fn spec(&self) -> ToolSpec; fn call(&self, args: &Args) -> Result<String, String>; }`
   and a `ToolRegistry` holding `HashMap<String, Box<dyn Tool>>`.
4. Implement `run(llm, tools, query, max_steps) -> Result<RunResult, AgentError>`:
   - push the assistant message to history every step;
   - if there are no tool calls, return the text;
   - otherwise dispatch every call; convert `Err` into `ToolResult { is_error: true }` — never panic or propagate;
   - after `max_steps`, return `AgentError::StepLimitExceeded`.
5. Write tests first, using a scripted `Llm`: happy path, tool error returned as data,
   unknown tool, step limit.

Rules: no `unwrap()` on model or tool output; every tool validates its own arguments;
print or log the full trace (call id, tool, args, result, error flag) for debugging.
