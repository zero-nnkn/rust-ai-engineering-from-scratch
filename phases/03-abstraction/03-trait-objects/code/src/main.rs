//! Phase 03 · Lesson 03 — Trait objects & dynamic dispatch (dyn Tool)
//!
//! Run:  cargo run -p trait-objects
//! Test: cargo test -p trait-objects
//!
//! Scenario: an agent's tool registry. Tools are different types, decided at
//! runtime (from config, from what the model asks for), stored in one collection.
//! That's what `Box<dyn Tool>` is for.

use std::collections::BTreeMap;

/// The interface every tool provides.
trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, input: &str) -> Result<String, String>;
}

struct Calculator;

impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }
    fn description(&self) -> &str {
        "Evaluate `a OP b` where OP is + - * /"
    }
    fn run(&self, input: &str) -> Result<String, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let [a, op, b] = parts[..] else {
            return Err(format!("expected `a OP b`, got {input:?}"));
        };
        let a: f64 = a.parse().map_err(|_| format!("not a number: {a}"))?;
        let b: f64 = b.parse().map_err(|_| format!("not a number: {b}"))?;
        let r = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" if b == 0.0 => return Err("division by zero".into()),
            "/" => a / b,
            _ => return Err(format!("unknown operator {op}")),
        };
        Ok(r.to_string())
    }
}

/// A tool with configuration: it holds data, unlike the unit struct above.
struct Lookup {
    tool_name: String,
    facts: BTreeMap<String, String>,
}

impl Tool for Lookup {
    fn name(&self) -> &str {
        &self.tool_name
    }
    fn description(&self) -> &str {
        "Look up a fact by key"
    }
    fn run(&self, input: &str) -> Result<String, String> {
        self.facts
            .get(input.trim())
            .cloned()
            .ok_or_else(|| format!("no fact for {input:?}"))
    }
}

/// The registry owns its tools. Each Box<dyn Tool> is a pointer to the tool's data
/// plus a pointer to a table of its methods (the vtable).
#[derive(Default)]
struct Registry {
    tools: Vec<Box<dyn Tool>>,
}

impl Registry {
    /// Generic on the way in (any concrete Tool), dynamic once stored.
    fn register(&mut self, tool: impl Tool + 'static) -> &mut Self {
        self.tools.push(Box::new(tool));
        self
    }

    fn find(&self, name: &str) -> Option<&dyn Tool> {
        self.tools
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }

    fn call(&self, name: &str, input: &str) -> Result<String, String> {
        match self.find(name) {
            Some(tool) => tool.run(input), // dynamic dispatch: which `run` is decided at runtime
            None => Err(format!("unknown tool {name:?}")),
        }
    }

    /// The text you'd put in a system prompt so the model knows the tools.
    fn manifest(&self) -> String {
        self.tools
            .iter()
            .map(|t| format!("- {}: {}", t.name(), t.description()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ---- A second use: a pipeline of transforms, configured at runtime ------------
trait Transform {
    fn apply(&self, text: String) -> String;
}

struct Trim;
struct Lowercase;
struct Truncate(usize);

impl Transform for Trim {
    fn apply(&self, text: String) -> String {
        text.trim().to_string()
    }
}
impl Transform for Lowercase {
    fn apply(&self, text: String) -> String {
        text.to_lowercase()
    }
}
impl Transform for Truncate {
    fn apply(&self, text: String) -> String {
        text.chars().take(self.0).collect()
    }
}

/// Build a pipeline from names, e.g. read from a config file.
fn pipeline_from_config(names: &[&str]) -> Result<Vec<Box<dyn Transform>>, String> {
    names
        .iter()
        .map(|n| -> Result<Box<dyn Transform>, String> {
            match *n {
                "trim" => Ok(Box::new(Trim)),
                "lower" => Ok(Box::new(Lowercase)),
                s if s.starts_with("truncate:") => {
                    let n = s["truncate:".len()..]
                        .parse()
                        .map_err(|_| format!("bad truncate length in {s:?}"))?;
                    Ok(Box::new(Truncate(n)))
                }
                other => Err(format!("unknown transform {other:?}")),
            }
        })
        .collect()
}

fn run_pipeline(steps: &[Box<dyn Transform>], text: &str) -> String {
    steps
        .iter()
        .fold(text.to_string(), |acc, step| step.apply(acc))
}

fn main() {
    let mut registry = Registry::default();
    registry.register(Calculator).register(Lookup {
        tool_name: "facts".into(),
        facts: [("rust_release".to_string(), "2015".to_string())].into(),
    });

    println!("manifest:\n{}", registry.manifest());
    for (tool, input) in [
        ("calculator", "6 * 7"),
        ("facts", "rust_release"),
        ("calculator", "1 / 0"),
        ("search", "anything"),
    ] {
        println!("call {tool}({input:?}) → {:?}", registry.call(tool, input));
    }

    match pipeline_from_config(&["trim", "lower", "truncate:12"]) {
        Ok(steps) => println!(
            "pipeline → {:?}",
            run_pipeline(&steps, "   Hello FROM The Prompt Preprocessor   ")
        ),
        Err(e) => println!("config error: {e}"),
    }
    println!(
        "bad config → {:?}",
        pipeline_from_config(&["trim", "shout"]).err()
    );

    println!(
        "size of &Calculator = {} bytes, &dyn Tool = {} bytes (data ptr + vtable ptr)",
        std::mem::size_of::<&Calculator>(),
        std::mem::size_of::<&dyn Tool>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculator_parses_and_rejects() {
        assert_eq!(Calculator.run("2 + 3"), Ok("5".into()));
        assert!(Calculator.run("2 plus 3").is_err());
        assert!(Calculator.run("2 +").is_err());
    }

    #[test]
    fn unknown_tool_is_an_error() {
        let r = Registry::default();
        assert_eq!(r.call("x", ""), Err("unknown tool \"x\"".into()));
    }

    #[test]
    fn pipeline_applies_in_order() {
        let steps = pipeline_from_config(&["truncate:3", "lower"]).unwrap();
        assert_eq!(run_pipeline(&steps, "ABCDEF"), "abc");
    }

    #[test]
    fn bad_truncate_is_reported() {
        assert!(pipeline_from_config(&["truncate:x"]).is_err());
    }
}
