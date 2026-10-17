//! Phase 03 · Lesson 04 — Closures & function types
//!
//! Run:  cargo run -p closures
//! Test: cargo test -p closures
//!
//! Scenario: the callback-heavy parts of an LLM client:
//!   on_token     a callback for every streamed token        (FnMut: it updates state)
//!   should_retry a policy passed in by the caller            (Fn: it only reads)
//!   on_complete  runs once with the finished text            (FnOnce: it consumes)
//!   make_*       functions that return configured closures   (impl Fn / Box<dyn Fn>)

/// Simulate a streaming response: call `on_token` for each token, then
/// `on_complete` once with the full text.
fn stream<T, C>(tokens: &[&str], mut on_token: T, on_complete: C)
where
    T: FnMut(&str),    // may mutate what it captured; called many times
    C: FnOnce(String), // may consume what it captured; called at most once
{
    let mut full = String::new();
    for t in tokens {
        on_token(t);
        full.push_str(t);
    }
    on_complete(full);
}

/// Retry `attempt` until it succeeds or `should_retry` says stop.
/// `should_retry` is Fn: it's called many times and must not mutate anything.
fn with_retries<F, P>(mut attempt: F, should_retry: P, max: u32) -> Result<String, String>
where
    F: FnMut(u32) -> Result<String, String>,
    P: Fn(&str) -> bool,
{
    let mut last_err = String::from("never ran");
    for n in 1..=max {
        match attempt(n) {
            Ok(v) => return Ok(v),
            Err(e) if should_retry(&e) => last_err = e,
            Err(e) => return Err(e),
        }
    }
    Err(format!("gave up after {max}: {last_err}"))
}

/// Return a closure that remembers its configuration. `move` copies/moves the
/// captured values into the closure so it can outlive this function.
fn make_redactor(secrets: Vec<String>) -> impl Fn(&str) -> String {
    move |text| {
        secrets
            .iter()
            .fold(text.to_string(), |acc, s| acc.replace(s.as_str(), "***"))
    }
}

/// A list of closures of *different* types needs a common type: Box<dyn Fn>.
fn middleware() -> Vec<Box<dyn Fn(String) -> String>> {
    let max_len = 40;
    vec![
        Box::new(|s| s.trim().to_string()),
        Box::new(|s| s.replace("\n", " ")),
        Box::new(move |s| s.chars().take(max_len).collect()),
    ]
}

fn main() {
    // FnMut: the closure mutates `count` and `longest`, which it borrows mutably.
    let mut count = 0;
    let mut longest = String::new();
    let mut transcript = Vec::new();
    stream(
        &["Rust ", "closures ", "capture ", "their ", "environment."],
        |tok| {
            count += 1;
            if tok.len() > longest.len() {
                longest = tok.to_string();
            }
        },
        |full| transcript.push(full), // FnOnce: `full` is moved into the Vec
    );
    println!("tokens        : {count}");
    println!("longest token : {longest:?}");
    println!("transcript    : {transcript:?}");

    // Fn passed as a policy; FnMut for an attempt counter.
    let mut calls = Vec::new();
    let result = with_retries(
        |n| {
            calls.push(n);
            if n < 3 {
                Err("503 overloaded".into())
            } else {
                Ok("answer".into())
            }
        },
        |err| err.starts_with("503") || err.starts_with("429"),
        5,
    );
    println!("retries       : {result:?} after attempts {calls:?}");

    let non_retryable = with_retries(|_| Err("400 bad request".into()), |e| e.starts_with("5"), 5);
    println!("no retry      : {non_retryable:?}");

    // A function pointer works anywhere a closure does, if it matches.
    let is_rate_limit: fn(&str) -> bool = |e| e.contains("429");
    println!("fn pointer    : {}", is_rate_limit("HTTP 429"));

    // Returned closure with captured state
    let redact = make_redactor(vec!["sk-live-123".into(), "hunter2".into()]);
    println!("redacted      : {}", redact("key=sk-live-123 pw=hunter2"));

    // Box<dyn Fn> pipeline
    let text = "  A long prompt\nwith a newline and far too many words in it  ".to_string();
    let cleaned = middleware().iter().fold(text, |acc, step| step(acc));
    println!("middleware    : {cleaned:?}");

    // Closures with iterator methods (next lesson goes deeper)
    let mut scores: Vec<(&str, f64)> = vec![("b", 0.4), ("a", 0.9), ("c", 0.7)];
    scores.sort_by(|x, y| y.1.total_cmp(&x.1));
    println!("sort_by       : {scores:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_calls_on_token_per_token_and_complete_once() {
        let mut seen = 0;
        let mut done = None;
        stream(&["a", "b"], |_| seen += 1, |full| done = Some(full));
        assert_eq!(seen, 2);
        assert_eq!(done.as_deref(), Some("ab"));
    }

    #[test]
    fn retries_stop_at_max() {
        let r = with_retries(|_| Err("503".into()), |_| true, 3);
        assert_eq!(r, Err("gave up after 3: 503".into()));
    }

    #[test]
    fn redactor_outlives_its_inputs() {
        let redact = {
            let secrets = vec!["x".to_string()];
            make_redactor(secrets) // `secrets` moved into the closure
        };
        assert_eq!(redact("axb"), "a***b");
    }

    #[test]
    fn middleware_runs_in_order() {
        let out = middleware()
            .iter()
            .fold(" a\nb ".to_string(), |acc, f| f(acc));
        assert_eq!(out, "a b");
    }
}
