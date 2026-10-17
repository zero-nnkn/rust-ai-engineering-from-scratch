//! Phase 01 · Lesson 04 — Enums & pattern matching
//!
//! Run:  cargo run -p enums-and-match
//! Test: cargo test -p enums-and-match
//!
//! Scenario: the events an LLM streaming API sends. Each kind of event carries
//! different data, which is exactly what Rust enums are for.

/// A role can only be one of these. A typo like "usr" no longer compiles.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    System,
    User,
    Assistant,
}

/// Why the model stopped generating.
#[derive(Debug, Clone, PartialEq)]
enum StopReason {
    EndTurn,
    MaxTokens,
    ToolUse { tool: String },
}

/// One event from a streaming response. Each variant has its own fields.
#[derive(Debug, Clone, PartialEq)]
enum StreamEvent {
    Start { model: String },
    TextDelta(String),
    Usage { input: u32, output: u32 },
    Stop(StopReason),
    Error { status: u16, message: String },
}

impl Role {
    fn as_str(self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }

    /// Parsing can fail, so it returns Option (Lesson 05 goes deeper).
    fn parse(s: &str) -> Option<Role> {
        match s {
            "system" => Some(Role::System),
            "user" => Some(Role::User),
            "assistant" => Some(Role::Assistant),
            _ => None,
        }
    }
}

/// The finished result of folding a stream of events.
#[derive(Debug, Default, PartialEq)]
struct Response {
    model: String,
    text: String,
    tokens: u32,
    stop: Option<StopReason>,
    error: Option<String>,
}

fn collect(events: &[StreamEvent]) -> Response {
    let mut r = Response::default();
    for event in events {
        // `match` must handle every variant. Add a variant and this stops compiling
        // until you decide what to do with it.
        match event {
            StreamEvent::Start { model } => r.model = model.clone(),
            StreamEvent::TextDelta(chunk) => r.text.push_str(chunk),
            StreamEvent::Usage { input, output } => r.tokens = input + output,
            StreamEvent::Stop(reason) => r.stop = Some(reason.clone()),
            // A guard (`if ...`) refines a pattern.
            StreamEvent::Error { status, message } if *status == 429 => {
                r.error = Some(format!("rate limited: {message}"));
            }
            StreamEvent::Error { status, message } => {
                r.error = Some(format!("HTTP {status}: {message}"));
            }
        }
    }
    r
}

/// Short summary of how a response ended.
fn describe(stop: &Option<StopReason>) -> String {
    match stop {
        None => "still streaming (no stop event)".to_string(),
        Some(StopReason::EndTurn) => "finished normally".to_string(),
        Some(StopReason::MaxTokens) => "cut off: raise max_tokens".to_string(),
        // Bind a field from inside a nested pattern.
        Some(StopReason::ToolUse { tool }) => format!("wants to call tool `{tool}`"),
    }
}

fn main() {
    let events = vec![
        StreamEvent::Start {
            model: "small-model".into(),
        },
        StreamEvent::TextDelta("Ownership ".into()),
        StreamEvent::TextDelta("means one owner.".into()),
        StreamEvent::Usage {
            input: 12,
            output: 5,
        },
        StreamEvent::Stop(StopReason::EndTurn),
    ];
    let response = collect(&events);
    println!("model : {}", response.model);
    println!("text  : {}", response.text);
    println!("tokens: {}", response.tokens);
    println!("stop  : {}", describe(&response.stop));

    // `if let`: match one pattern, ignore the rest. Python: `if isinstance(e, X):`
    for e in &events {
        if let StreamEvent::Usage { output, .. } = e {
            println!("if let: {output} output tokens");
        }
    }

    // `matches!`: a quick bool check against a pattern.
    let text_events = events
        .iter()
        .filter(|e| matches!(e, StreamEvent::TextDelta(_)))
        .count();
    println!("matches!: {text_events} text deltas");

    // `let ... else`: destructure or leave early.
    let raw_role = "assistant";
    let Some(role) = Role::parse(raw_role) else {
        println!("unknown role {raw_role}");
        return;
    };
    println!("role  : {:?} → {:?}", role, role.as_str());

    let failed = collect(&[StreamEvent::Error {
        status: 429,
        message: "slow down".into(),
    }]);
    println!("error : {:?}", failed.error);
    for stop in [
        StopReason::MaxTokens,
        StopReason::ToolUse {
            tool: "search".into(),
        },
    ] {
        println!("stop  : {}", describe(&Some(stop)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_text_in_order() {
        let r = collect(&[
            StreamEvent::TextDelta("a".into()),
            StreamEvent::TextDelta("b".into()),
        ]);
        assert_eq!(r.text, "ab");
        assert_eq!(r.stop, None);
    }

    #[test]
    fn rate_limit_gets_its_own_message() {
        let r = collect(&[StreamEvent::Error {
            status: 429,
            message: "x".into(),
        }]);
        assert_eq!(r.error.as_deref(), Some("rate limited: x"));
    }

    #[test]
    fn role_round_trips() {
        for role in [Role::System, Role::User, Role::Assistant] {
            assert_eq!(Role::parse(role.as_str()), Some(role));
        }
        assert_eq!(Role::parse("usr"), None);
    }
}
