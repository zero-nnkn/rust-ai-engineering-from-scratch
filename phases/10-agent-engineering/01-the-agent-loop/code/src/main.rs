//! Phase 10 · Lesson 01 — The agent loop
//!
//! Run:  cargo run -p agent-loop
//! Test: cargo test -p agent-loop
//!
//! A complete ReAct-style agent loop with no framework: messages, tool calls,
//! a tool registry, a step limit. The LLM is behind a trait, so this lesson runs
//! offline with a scripted model. In Phase 9 · Lesson 01 you implement the same
//! trait against a real API and drop it in without touching the loop.
//!
//! Zero dependencies: tool arguments are a simple string map here. Lesson 10-03
//! upgrades them to typed structs with serde_json + schemars.

use std::collections::{BTreeMap, HashMap};

/// Tool arguments, as the model would send them (already parsed from JSON).
pub type Args = BTreeMap<String, String>;

/// Build an `Args` map: `args(&[("a", "4"), ("op", "*")])`.
pub fn args(pairs: &[(&str, &str)]) -> Args {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

// ---------- Messages ----------

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    User(String),
    Assistant {
        text: Option<String>,
        tool_calls: Vec<ToolCall>,
    },
    ToolResult {
        call_id: String,
        content: String,
        is_error: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub args: Args,
}

// ---------- The model, behind a trait ----------

pub trait Llm {
    /// Given the conversation so far and the available tools, produce the next
    /// assistant message (text, tool calls, or both).
    fn complete(&mut self, history: &[Message], tools: &[ToolSpec]) -> Message;
}

#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    /// Parameter name → short type/description. A JSON Schema in lesson 10-03.
    pub params: Vec<(String, String)>,
}

// ---------- Tools ----------

pub trait Tool {
    fn spec(&self) -> ToolSpec;
    fn call(&self, args: &Args) -> Result<String, String>;
}

pub struct Calculator;

impl Tool for Calculator {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            name: "calculator".into(),
            description: "Apply +, -, * or / to two numbers.".into(),
            params: vec![
                ("a".into(), "number".into()),
                ("b".into(), "number".into()),
                ("op".into(), "one of + - * /".into()),
            ],
        }
    }

    fn call(&self, args: &Args) -> Result<String, String> {
        let num = |key: &str| -> Result<f64, String> {
            let raw = args.get(key).ok_or(format!("missing argument `{key}`"))?;
            raw.parse::<f64>()
                .map_err(|_| format!("`{key}` is not a number: {raw}"))
        };
        let (a, b) = (num("a")?, num("b")?);
        let result = match args
            .get("op")
            .map(String::as_str)
            .ok_or("missing argument `op`")?
        {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" if b == 0.0 => return Err("division by zero".into()),
            "/" => a / b,
            other => return Err(format!("unknown op `{other}`")),
        };
        Ok(result.to_string())
    }
}

pub struct WordCount;

impl Tool for WordCount {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            name: "word_count".into(),
            description: "Count the words in a text.".into(),
            params: vec![("text".into(), "string".into())],
        }
    }

    fn call(&self, args: &Args) -> Result<String, String> {
        let text = args.get("text").ok_or("missing argument `text`")?;
        Ok(text.split_whitespace().count().to_string())
    }
}

/// Tools are stored as trait objects: `Box<dyn Tool>` is Rust's answer to
/// "a dict of callables that all follow the same protocol".
#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register(mut self, tool: impl Tool + 'static) -> Self {
        self.tools.insert(tool.spec().name, Box::new(tool));
        self
    }

    pub fn specs(&self) -> Vec<ToolSpec> {
        let mut specs: Vec<_> = self.tools.values().map(|t| t.spec()).collect();
        specs.sort_by(|a, b| a.name.cmp(&b.name));
        specs
    }

    pub fn dispatch(&self, call: &ToolCall) -> Result<String, String> {
        match self.tools.get(&call.name) {
            Some(tool) => tool.call(&call.args),
            None => Err(format!("unknown tool `{}`", call.name)),
        }
    }
}

// ---------- The loop ----------

#[derive(Debug, PartialEq)]
pub enum AgentError {
    StepLimitExceeded(usize),
}

pub struct RunResult {
    pub answer: String,
    pub history: Vec<Message>,
    pub steps: usize,
}

pub fn run(
    llm: &mut impl Llm,
    tools: &ToolRegistry,
    query: &str,
    max_steps: usize,
) -> Result<RunResult, AgentError> {
    let specs = tools.specs();
    let mut history = vec![Message::User(query.to_string())];

    for step in 1..=max_steps {
        let msg = llm.complete(&history, &specs);
        history.push(msg.clone());

        let Message::Assistant { text, tool_calls } = msg else {
            unreachable!("Llm::complete must return an Assistant message");
        };

        if tool_calls.is_empty() {
            return Ok(RunResult {
                answer: text.unwrap_or_default(),
                history,
                steps: step,
            });
        }

        for call in &tool_calls {
            // Errors are fed back to the model as data, not raised:
            // the model gets a chance to correct itself.
            let (content, is_error) = match tools.dispatch(call) {
                Ok(out) => (out, false),
                Err(err) => (err, true),
            };
            history.push(Message::ToolResult {
                call_id: call.id.clone(),
                content,
                is_error,
            });
        }
    }

    Err(AgentError::StepLimitExceeded(max_steps))
}

// ---------- A scripted model, so the lesson runs offline ----------

/// Replays a fixed list of responses. Real models come in Phase 9.
pub struct ScriptedLlm {
    script: Vec<Message>,
}

impl ScriptedLlm {
    pub fn new(mut script: Vec<Message>) -> Self {
        script.reverse(); // so we can pop from the end in order
        Self { script }
    }
}

impl Llm for ScriptedLlm {
    fn complete(&mut self, _history: &[Message], _tools: &[ToolSpec]) -> Message {
        // A real model reads `history` and `tools`; the script just replays.
        self.script.pop().unwrap_or(Message::Assistant {
            text: Some("I have nothing more to add.".into()),
            tool_calls: vec![],
        })
    }
}

fn call(id: &str, name: &str, args: Args) -> ToolCall {
    ToolCall {
        id: id.into(),
        name: name.into(),
        args,
    }
}

fn demo_script() -> Vec<Message> {
    vec![
        Message::Assistant {
            text: Some("I'll count the words, then multiply.".into()),
            tool_calls: vec![call(
                "c1",
                "word_count",
                args(&[("text", "agents are just loops")]),
            )],
        },
        Message::Assistant {
            text: None,
            tool_calls: vec![call(
                "c2",
                "calculator",
                args(&[("a", "4"), ("b", "25"), ("op", "*")]),
            )],
        },
        Message::Assistant {
            text: Some("The sentence has 4 words; 4 × 25 = 100.".into()),
            tool_calls: vec![],
        },
    ]
}

fn main() {
    let tools = ToolRegistry::default()
        .register(Calculator)
        .register(WordCount);
    println!(
        "tools: {:?}",
        tools.specs().iter().map(|s| &s.name).collect::<Vec<_>>()
    );

    let mut llm = ScriptedLlm::new(demo_script());
    let query = "How many words are in 'agents are just loops', times 25?";
    println!("user: {query}");

    match run(&mut llm, &tools, query, 8) {
        Ok(result) => {
            // Print the full trace in order. Reading traces is the #1 agent-debugging skill.
            for m in &result.history[1..] {
                match m {
                    Message::Assistant { text, tool_calls } => {
                        if let Some(t) = text {
                            println!("assistant: {t}");
                        }
                        for c in tool_calls {
                            println!("  → call {} {}({:?})", c.id, c.name, c.args);
                        }
                    }
                    Message::ToolResult {
                        call_id,
                        content,
                        is_error,
                    } => {
                        let tag = if *is_error { "ERROR" } else { "ok" };
                        println!("  ← result {call_id} [{tag}]: {content}");
                    }
                    Message::User(u) => println!("user: {u}"),
                }
            }
            println!("done in {} steps", result.steps);
        }
        Err(e) => eprintln!("agent failed: {e:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> ToolRegistry {
        ToolRegistry::default()
            .register(Calculator)
            .register(WordCount)
    }

    #[test]
    fn completes_demo_script() {
        let mut llm = ScriptedLlm::new(demo_script());
        let r = run(&mut llm, &registry(), "q", 8).unwrap();
        assert_eq!(r.steps, 3);
        assert!(r.answer.contains("100"));
    }

    #[test]
    fn tool_errors_are_returned_to_model_not_raised() {
        let script = vec![
            Message::Assistant {
                text: None,
                tool_calls: vec![call(
                    "x",
                    "calculator",
                    args(&[("a", "1"), ("b", "0"), ("op", "/")]),
                )],
            },
            Message::Assistant {
                text: Some("Cannot divide by zero.".into()),
                tool_calls: vec![],
            },
        ];
        let r = run(&mut ScriptedLlm::new(script), &registry(), "q", 8).unwrap();
        assert!(r.history.iter().any(|m| matches!(
            m,
            Message::ToolResult { is_error: true, content, .. } if content == "division by zero"
        )));
    }

    #[test]
    fn unknown_tool_is_an_error_result() {
        let r = registry().dispatch(&call("x", "rm_rf", args(&[])));
        assert_eq!(r, Err("unknown tool `rm_rf`".into()));
    }

    #[test]
    fn step_limit_stops_runaway_agents() {
        let looping: Vec<Message> = (0..20)
            .map(|i| Message::Assistant {
                text: None,
                tool_calls: vec![call(
                    &i.to_string(),
                    "word_count",
                    args(&[("text", "again")]),
                )],
            })
            .collect();
        let r = run(&mut ScriptedLlm::new(looping), &registry(), "q", 5);
        assert_eq!(r.err(), Some(AgentError::StepLimitExceeded(5)));
    }
}
