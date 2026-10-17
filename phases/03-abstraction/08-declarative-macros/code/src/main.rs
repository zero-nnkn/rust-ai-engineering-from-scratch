//! Phase 03 · Lesson 08 — macro_rules! basics
//!
//! Run:  cargo run -p declarative-macros
//! Test: cargo test -p declarative-macros
//!
//! Scenario: removing boilerplate from LLM code:
//!   messages![...]   build a conversation with a readable syntax
//!   string_map!{...} a HashMap literal, like a Python dict literal
//!   tool_enum!       generate an enum, its name() and from_name() from one list
//!   ensure!          early-return an error when a condition fails

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: &'static str,
    pub content: String,
}

/// `messages![system: "...", user: "...", assistant: "..."]`
/// $(...),* repeats a pattern separated by commas; $(,)? allows a trailing comma.
macro_rules! messages {
    ($($role:ident : $content:expr),* $(,)?) => {
        vec![$(Message {
            role: stringify!($role),          // turns the identifier into a &str
            content: ($content).to_string(),
        }),*]
    };
}

/// A HashMap literal: `string_map! { "a" => 1, "b" => 2 }`
macro_rules! string_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(map.insert($key.to_string(), $value);)*
        map
    }};
}

/// Return early with an error if a condition doesn't hold (like anyhow's `ensure!`).
macro_rules! ensure {
    ($cond:expr, $($fmt:tt)+) => {
        if !$cond {
            return Err(format!($($fmt)+));
        }
    };
}

/// Generate an enum plus string conversions from one list, so they can't drift apart.
macro_rules! tool_enum {
    ($name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $name { $($variant),+ }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            pub fn name(self) -> &'static str {
                match self { $($name::$variant => $text),+ }
            }

            pub fn from_name(s: &str) -> Option<Self> {
                match s { $($text => Some($name::$variant),)+ _ => None }
            }
        }
    };
}

tool_enum!(ToolName {
    Search => "search",
    Calculator => "calculator",
    ReadFile => "read_file",
});

fn validate(temperature: f64, max_tokens: u32) -> Result<(), String> {
    ensure!(
        (0.0..=2.0).contains(&temperature),
        "temperature {temperature} out of range"
    );
    ensure!(max_tokens > 0, "max_tokens must be positive");
    Ok(())
}

fn main() {
    let convo = messages![
        system: "You are terse.",
        user: "Name a Rust macro.",
        assistant: "println!",
    ];
    for m in &convo {
        println!("messages! : {:>9}: {}", m.role, m.content);
    }

    let prices = string_map! { "input" => 3.0, "output" => 15.0 };
    let mut keys: Vec<_> = prices.keys().collect();
    keys.sort();
    println!("string_map: {} entries, keys {keys:?}", prices.len());

    println!("ensure!   : {:?}", validate(0.7, 100));
    println!("ensure!   : {:?}", validate(3.0, 100));

    let names: Vec<&str> = ToolName::ALL.iter().map(|t| t.name()).collect();
    println!("tool_enum : {names:?}");
    println!(
        "tool_enum : {:?} / {:?}",
        ToolName::from_name("read_file"),
        ToolName::from_name("rm")
    );

    // Built-in macros you already use, for comparison
    let v = vec![1, 2, 3]; // vec!
    let s = format!("{v:?}"); // format!
    assert_eq!(s, "[1, 2, 3]"); // assert_eq!
    println!("built-in  : {s} (vec!, format!, assert_eq!, println!)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_macro_builds_in_order() {
        let m = messages![user: "a", assistant: format!("b{}", 1)];
        assert_eq!(m.len(), 2);
        assert_eq!(
            m[1],
            Message {
                role: "assistant",
                content: "b1".into()
            }
        );
    }

    #[test]
    fn empty_messages_is_empty_vec() {
        let m: Vec<Message> = messages![];
        assert!(m.is_empty());
    }

    #[test]
    fn tool_enum_round_trips() {
        for t in ToolName::ALL {
            assert_eq!(ToolName::from_name(t.name()), Some(*t));
        }
    }

    #[test]
    fn ensure_returns_early() {
        assert_eq!(validate(1.0, 0), Err("max_tokens must be positive".into()));
    }
}
