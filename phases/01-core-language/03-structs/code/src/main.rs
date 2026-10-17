//! Phase 01 · Lesson 03 — Structs & methods (classes without inheritance)
//!
//! Run:  cargo run -p structs
//! Test: cargo test -p structs
//!
//! Scenario: a chat conversation with a token budget, the core state of any
//! LLM application.

use std::fmt;

// A plain struct: named fields, like a @dataclass.
// `derive` generates common behaviour, like dataclass(eq=True, repr=True).
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub tokens: u32,
}

// A tuple struct: a named wrapper around one value (the "newtype" pattern).
// A ModelName can't be passed where any old String is expected.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelName(pub String);

#[derive(Debug, Default)]
pub struct Conversation {
    model: Option<ModelName>,
    max_tokens: u32,
    messages: Vec<Message>,
}

impl Message {
    // An associated function (no `self`): Rust's constructor convention is `new`.
    // Python: a @classmethod or __init__.
    pub fn new(role: &str, content: &str) -> Self {
        Message {
            role: role.to_string(),
            content: content.to_string(),
            // Rough estimate: ~4 characters per token, at least 1.
            tokens: (content.chars().count() as u32).div_ceil(4).max(1),
        }
    }
}

impl Conversation {
    pub fn new(model: &str, max_tokens: u32) -> Self {
        Conversation {
            model: Some(ModelName(model.to_string())),
            max_tokens,
            ..Default::default() // fill the remaining fields from Default
        }
    }

    // &self: read-only access. Python's `self`, but promising not to modify.
    pub fn total_tokens(&self) -> u32 {
        self.messages.iter().map(|m| m.tokens).sum()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    // &mut self: this method changes the conversation.
    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Drop the oldest non-system messages until the conversation fits the budget.
    /// Returns how many messages were removed.
    pub fn trim_to_budget(&mut self) -> usize {
        let mut removed = 0;
        while self.total_tokens() > self.max_tokens {
            match self.messages.iter().position(|m| m.role != "system") {
                Some(i) => {
                    self.messages.remove(i);
                    removed += 1;
                }
                None => break, // only system messages left: nothing more we may drop
            }
        }
        removed
    }

    // self (by value): consumes the conversation. After calling this, the caller
    // can't use it any more. Useful for "finish and hand over" operations.
    pub fn into_messages(self) -> Vec<Message> {
        self.messages
    }
}

// Display is __str__: how the value prints with "{}".
impl fmt::Display for Conversation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model = match &self.model {
            Some(ModelName(name)) => name.as_str(),
            None => "<no model>",
        };
        writeln!(
            f,
            "[{model}] {} messages, {}/{} tokens",
            self.len(),
            self.total_tokens(),
            self.max_tokens
        )?;
        for m in &self.messages {
            writeln!(f, "  {:>9}: {}", m.role, m.content)?;
        }
        Ok(())
    }
}

fn main() {
    let mut convo = Conversation::new("small-model", 30);
    convo.push(Message::new("system", "You are a concise assistant."));
    convo.push(Message::new("user", "What is ownership in Rust?"));
    convo.push(Message::new(
        "assistant",
        "Every value has exactly one owner; when the owner goes out of scope, it is dropped.",
    ));
    convo.push(Message::new("user", "And borrowing?"));

    print!("{convo}");
    let removed = convo.trim_to_budget();
    println!("trimmed {removed} message(s)");
    print!("{convo}");

    // Struct update syntax: copy the rest of the fields from another value.
    let first = Message::new("user", "hi");
    let edited = Message {
        content: "hello".to_string(),
        ..first.clone()
    };
    println!("{:?}", edited);
    println!("equal? {}", first == edited); // PartialEq from derive

    // `default()` works because Conversation derives Default.
    let blank = Conversation::default();
    print!("{blank}");

    let messages = convo.into_messages();
    println!("handed over {} messages", messages.len());
    // print!("{convo}"); // ← uncomment: convo was moved by into_messages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_estimate_rounds_up_and_is_at_least_one() {
        assert_eq!(Message::new("user", "").tokens, 1);
        assert_eq!(Message::new("user", "abcde").tokens, 2);
    }

    #[test]
    fn trim_keeps_system_messages() {
        let mut c = Conversation::new("m", 5);
        c.push(Message::new("system", "rules rules rules")); // 5 tokens
        c.push(Message::new("user", "a long question here")); // 5 tokens
        assert_eq!(c.trim_to_budget(), 1);
        assert_eq!(c.into_messages()[0].role, "system");
    }

    #[test]
    fn trim_stops_when_only_system_left() {
        let mut c = Conversation::new("m", 1);
        c.push(Message::new("system", "a very long system prompt"));
        assert_eq!(c.trim_to_budget(), 0);
        assert_eq!(c.len(), 1);
    }
}
