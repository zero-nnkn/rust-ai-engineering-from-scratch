//! Phase 03 · Lesson 06 — Error handling (by hand, then with thiserror & anyhow)
//!
//! Run:  cargo run -p error-handling
//! Test: cargo test -p error-handling
//!
//! Scenario: loading a model config file (`key = value` lines). Things that can
//! go wrong: the file can't be read (io), a number doesn't parse, a value is out
//! of range, a required key is missing. Each layer adds context.
//!
//! Everything here uses only std. The docs show the same code with the
//! `thiserror` (libraries) and `anyhow` (applications) crates.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct ModelConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

// ---- Library error type: an enum, one variant per failure the caller may handle --
#[derive(Debug)]
pub enum ConfigError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Syntax {
        line: usize,
        text: String,
    },
    BadNumber {
        key: &'static str,
        source: ParseFloatError,
    },
    OutOfRange {
        key: &'static str,
        value: f64,
        min: f64,
        max: f64,
    },
    Missing(&'static str),
}

// Display: the message for humans. Don't repeat the source's message here;
// it's reachable through `source()`.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io { path, .. } => write!(f, "cannot read config file {}", path.display()),
            ConfigError::Syntax { line, text } => {
                write!(f, "line {line}: expected `key = value`, got {text:?}")
            }
            ConfigError::BadNumber { key, .. } => write!(f, "`{key}` is not a number"),
            ConfigError::OutOfRange {
                key,
                value,
                min,
                max,
            } => write!(f, "`{key}` = {value} is outside {min}..={max}"),
            ConfigError::Missing(key) => write!(f, "missing required key `{key}`"),
        }
    }
}

// std::error::Error: makes it a "real" error, and exposes the underlying cause.
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Io { source, .. } => Some(source),
            ConfigError::BadNumber { source, .. } => Some(source),
            _ => None,
        }
    }
}

fn parse_pairs(text: &str) -> Result<HashMap<&str, &str>, ConfigError> {
    let mut pairs = HashMap::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line.split_once('=').ok_or_else(|| ConfigError::Syntax {
            line: i + 1,
            text: raw.to_string(),
        })?;
        pairs.insert(k.trim(), v.trim());
    }
    Ok(pairs)
}

fn number(
    pairs: &HashMap<&str, &str>,
    key: &'static str,
    min: f64,
    max: f64,
) -> Result<f64, ConfigError> {
    let raw = pairs.get(key).ok_or(ConfigError::Missing(key))?;
    let value: f64 = raw
        .parse()
        .map_err(|source| ConfigError::BadNumber { key, source })?;
    if !(min..=max).contains(&value) {
        return Err(ConfigError::OutOfRange {
            key,
            value,
            min,
            max,
        });
    }
    Ok(value)
}

pub fn parse_config(text: &str) -> Result<ModelConfig, ConfigError> {
    let pairs = parse_pairs(text)?;
    let model = pairs
        .get("model")
        .filter(|m| !m.is_empty())
        .ok_or(ConfigError::Missing("model"))?
        .to_string();
    let temperature = number(&pairs, "temperature", 0.0, 2.0)? as f32;
    let max_tokens = number(&pairs, "max_tokens", 1.0, 200_000.0)?;
    if max_tokens.fract() != 0.0 {
        return Err(ConfigError::OutOfRange {
            key: "max_tokens",
            value: max_tokens,
            min: 1.0,
            max: 200_000.0,
        });
    }
    Ok(ModelConfig {
        model,
        temperature,
        max_tokens: max_tokens as u32, // safe: whole number within 1..=200_000, checked above
    })
}

pub fn load_config(path: &Path) -> Result<ModelConfig, ConfigError> {
    let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    parse_config(&text)
}

// ---- Application layer: one error type for "anything", plus context --------------
// This is what `anyhow` gives you. By hand: Box<dyn Error> + a small wrapper.
#[derive(Debug)]
struct Context {
    message: String,
    source: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for Context {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// Add a message on top of any error: `result.context("while ...")`.
trait ResultExt<T> {
    fn context(self, message: &str) -> Result<T, Context>;
}

impl<T, E: Error + Send + Sync + 'static> ResultExt<T> for Result<T, E> {
    fn context(self, message: &str) -> Result<T, Context> {
        self.map_err(|e| Context {
            message: message.to_string(),
            source: Box::new(e),
        })
    }
}

/// Print an error and its whole chain of causes, like anyhow's `{:#}`.
fn report(err: &dyn Error) -> String {
    let mut out = err.to_string();
    let mut cause = err.source();
    while let Some(c) = cause {
        out.push_str(&format!("\n  caused by: {c}"));
        cause = c.source();
    }
    out
}

fn start_agent(config_path: &Path) -> Result<String, Box<dyn Error>> {
    let config = load_config(config_path).context("could not start the agent")?;
    Ok(format!(
        "agent ready: {} (t={}, max={})",
        config.model, config.temperature, config.max_tokens
    ))
}

fn main() {
    let good = "# model settings\nmodel = small-model\ntemperature = 0.2\nmax_tokens = 1024\n";
    println!("ok       : {:?}", parse_config(good));

    let cases = [
        "model = m\ntemperature 0.2\nmax_tokens = 5",
        "model = m\ntemperature = hot\nmax_tokens = 5",
        "model = m\ntemperature = 3.5\nmax_tokens = 5",
        "temperature = 0.5\nmax_tokens = 5",
    ];
    for text in cases {
        match parse_config(text) {
            Ok(c) => println!("ok       : {c:?}"),
            Err(e) => println!("error    : {}", report(&e)),
        }
    }

    // Matching on a specific variant: the caller can react differently.
    match parse_config("model = m\ntemperature = 9\nmax_tokens = 5") {
        Err(ConfigError::OutOfRange { key, max, .. }) => {
            println!("handled  : clamping `{key}` to {max}")
        }
        other => println!("other    : {other:?}"),
    }

    // Application layer: context + chain
    match start_agent(Path::new("/definitely/missing/agent.conf")) {
        Ok(msg) => println!("{msg}"),
        Err(e) => println!("app error: {}", report(e.as_ref())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_good_config_with_comments_and_spaces() {
        let c = parse_config("  # hi\nmodel=m\n temperature = 1.0 \nmax_tokens= 10\n").unwrap();
        assert_eq!(
            c,
            ModelConfig {
                model: "m".into(),
                temperature: 1.0,
                max_tokens: 10
            }
        );
    }

    #[test]
    fn bad_number_keeps_its_source() {
        let e = parse_config("model=m\ntemperature=hot\nmax_tokens=1").unwrap_err();
        assert!(matches!(
            e,
            ConfigError::BadNumber {
                key: "temperature",
                ..
            }
        ));
        assert!(e.source().is_some(), "the ParseFloatError is the cause");
    }

    #[test]
    fn fractional_max_tokens_is_rejected() {
        let e = parse_config("model=m\ntemperature=1\nmax_tokens=10.5").unwrap_err();
        assert!(matches!(
            e,
            ConfigError::OutOfRange {
                key: "max_tokens",
                ..
            }
        ));
    }

    #[test]
    fn io_error_chain_is_reported() {
        let e = start_agent(Path::new("/no/such/file")).unwrap_err();
        let text = report(e.as_ref());
        assert!(text.starts_with("could not start the agent"));
        assert!(text.contains("caused by: cannot read config file /no/such/file"));
        assert_eq!(text.lines().count(), 3, "{text}");
    }
}
