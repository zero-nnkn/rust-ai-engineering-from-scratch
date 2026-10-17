//! Phase 03 · Lesson 07 — Newtype, builder, typestate
//!
//! Run:  cargo run -p design-patterns
//! Test: cargo test -p design-patterns
//!
//! Scenario: building and sending a chat request to an LLM API.
//!   newtype    ApiKey (never printed), Tokens (can't be mixed up with other numbers)
//!   builder    ChatRequest::builder()...build() with validation
//!   typestate  Draft → Approved → Sent: the compiler enforces the order

use std::fmt;
use std::marker::PhantomData;

// ---------------- Newtype ------------------------------------------------------
/// A secret. Debug and Display never show it, so it can't leak into logs.
#[derive(Clone)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(raw: &str) -> Result<Self, String> {
        if raw.chars().count() < 8 {
            return Err("API key looks too short".into());
        }
        Ok(ApiKey(raw.to_string()))
    }
    /// The only way to get the secret out, and it's easy to search for in review.
    fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Show the last 2 *characters*. Slicing bytes (&self.0[len - 2..]) would panic
        // if the key ended in a multi-byte character.
        let n = self.0.chars().count();
        let tail: String = self.0.chars().skip(n.saturating_sub(2)).collect();
        write!(f, "ApiKey(***{tail})")
    }
}

/// A token count. A plain u32 could be confused with a character count, a byte
/// count, or a price. A newtype can't.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tokens(pub u32);

impl std::ops::Add for Tokens {
    type Output = Tokens;
    fn add(self, other: Tokens) -> Tokens {
        Tokens(self.0.saturating_add(other.0))
    }
}

// ---------------- Builder ------------------------------------------------------
#[derive(Debug)]
pub struct ChatRequest {
    model: String,
    system: Option<String>,
    messages: Vec<(String, String)>,
    max_tokens: Tokens,
    temperature: f32,
}

#[derive(Default)]
pub struct ChatRequestBuilder {
    model: Option<String>,
    system: Option<String>,
    messages: Vec<(String, String)>,
    max_tokens: Option<Tokens>,
    temperature: Option<f32>,
}

impl ChatRequest {
    pub fn builder() -> ChatRequestBuilder {
        ChatRequestBuilder::default()
    }
}

impl ChatRequestBuilder {
    // Each setter takes `self` by value and returns it, so calls chain.
    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }
    pub fn system(mut self, prompt: &str) -> Self {
        self.system = Some(prompt.to_string());
        self
    }
    pub fn user(mut self, text: &str) -> Self {
        self.messages.push(("user".into(), text.into()));
        self
    }
    pub fn max_tokens(mut self, n: u32) -> Self {
        self.max_tokens = Some(Tokens(n));
        self
    }
    pub fn temperature(mut self, t: f32) -> Self {
        self.temperature = Some(t);
        self
    }

    /// Validation happens once, here. A ChatRequest that exists is always valid.
    pub fn build(self) -> Result<ChatRequest, String> {
        let model = self.model.ok_or("model is required")?;
        if self.messages.is_empty() {
            return Err("at least one message is required".into());
        }
        let temperature = self.temperature.unwrap_or(1.0);
        if !(0.0..=2.0).contains(&temperature) {
            return Err(format!("temperature {temperature} is outside 0..=2"));
        }
        Ok(ChatRequest {
            model,
            system: self.system,
            messages: self.messages,
            max_tokens: self.max_tokens.unwrap_or(Tokens(1024)),
            temperature,
        })
    }
}

// ---------------- Typestate ------------------------------------------------------
// The state is part of the type. Methods exist only in the states where they make sense.
pub struct Draft;
pub struct Approved;
pub struct Sent;

pub struct Run<State> {
    request: ChatRequest,
    log: Vec<String>,
    _state: PhantomData<State>, // zero-sized: exists only for the type checker
}

impl Run<Draft> {
    pub fn new(request: ChatRequest) -> Self {
        Run {
            request,
            log: vec!["drafted".into()],
            _state: PhantomData,
        }
    }

    /// Human-in-the-loop approval. Consumes the Draft, returns an Approved run.
    pub fn approve(mut self, reviewer: &str) -> Run<Approved> {
        self.log.push(format!("approved by {reviewer}"));
        Run {
            request: self.request,
            log: self.log,
            _state: PhantomData,
        }
    }
}

impl Run<Approved> {
    /// `send` exists only on Approved runs: sending a Draft doesn't compile.
    pub fn send(mut self, key: &ApiKey) -> Run<Sent> {
        // A real client would make an HTTP call here, using key.expose().
        let auth_len = key.expose().len();
        self.log.push(format!(
            "sent {} message(s) to {} with a {auth_len}-char key",
            self.request.messages.len(),
            self.request.model
        ));
        Run {
            request: self.request,
            log: self.log,
            _state: PhantomData,
        }
    }
}

impl Run<Sent> {
    pub fn log(&self) -> &[String] {
        &self.log
    }
}

fn main() {
    let key = match ApiKey::new("sk-test-1234567890") {
        Ok(k) => k,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    println!("newtype   : {key:?}");
    println!("tokens    : {:?}", Tokens(1000) + Tokens(24));

    let request = ChatRequest::builder()
        .model("small-model")
        .system("Answer in one sentence.")
        .user("What is a newtype?")
        .temperature(0.3)
        .build();
    println!(
        "builder   : {:?}",
        request
            .as_ref()
            .map(|r| (&r.model, r.max_tokens, r.temperature, r.system.is_some()))
    );

    let bad = ChatRequest::builder()
        .model("m")
        .temperature(5.0)
        .user("hi")
        .build();
    println!("builder   : {:?}", bad.err());
    let empty = ChatRequest::builder().model("m").build();
    println!("builder   : {:?}", empty.err());

    if let Ok(request) = request {
        let sent = Run::new(request).approve("reviewer").send(&key);
        for line in sent.log() {
            println!("typestate : {line}");
        }
        // Run::new(...).send(&key)  ← doesn't compile: no `send` on Run<Draft>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_debug_hides_the_secret() {
        let k = ApiKey::new("sk-abcdefgh99").unwrap();
        let shown = format!("{k:?}");
        assert!(!shown.contains("abcdefgh"));
        assert!(shown.ends_with("99)"));
    }

    #[test]
    fn debug_is_safe_with_non_ascii_keys() {
        let k = ApiKey::new("khoá-bí-mật-ữ").unwrap();
        assert_eq!(format!("{k:?}"), "ApiKey(***-ữ)");
    }

    #[test]
    fn short_key_is_rejected() {
        assert!(ApiKey::new("abc").is_err());
    }

    #[test]
    fn builder_applies_defaults() {
        let r = ChatRequest::builder().model("m").user("x").build().unwrap();
        assert_eq!(r.max_tokens, Tokens(1024));
        assert_eq!(r.temperature, 1.0);
    }

    #[test]
    fn builder_requires_model() {
        assert_eq!(
            ChatRequest::builder().user("x").build().err().as_deref(),
            Some("model is required")
        );
    }

    #[test]
    fn tokens_add_saturates() {
        assert_eq!(Tokens(u32::MAX) + Tokens(1), Tokens(u32::MAX));
    }

    #[test]
    fn full_typestate_flow() {
        let req = ChatRequest::builder().model("m").user("x").build().unwrap();
        let key = ApiKey::new("12345678").unwrap();
        let sent = Run::new(req).approve("me").send(&key);
        assert_eq!(sent.log().len(), 3);
    }
}
