//! Phase 01 · Lesson 05 — Option & Result: no None surprises, no exceptions
//!
//! Run:  cargo run -p option-and-result
//! Test: cargo test -p option-and-result
//!
//! Scenario: parsing an LLM's tool call. The model's output is untrusted:
//! fields can be missing, numbers can be text, values can be out of range.

use std::collections::HashMap;
use std::fmt;

/// A validated search request, built from the model's raw key/value arguments.
#[derive(Debug, PartialEq)]
struct SearchCall {
    query: String,
    top_k: u32,
    min_score: Option<f64>, // optional argument: genuinely may be absent
}

/// Everything that can be wrong with the model's arguments.
#[derive(Debug, PartialEq)]
enum ArgError {
    Missing(&'static str),
    NotANumber { field: &'static str, value: String },
    OutOfRange { field: &'static str, value: f64 },
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgError::Missing(field) => write!(f, "missing argument `{field}`"),
            ArgError::NotANumber { field, value } => {
                write!(f, "`{field}` should be a number, got {value:?}")
            }
            ArgError::OutOfRange { field, value } => write!(f, "`{field}` out of range: {value}"),
        }
    }
}

type Args = HashMap<String, String>;

/// Option: "maybe there's a value". `get` returns Option<&String>.
fn optional<'a>(args: &'a Args, field: &str) -> Option<&'a str> {
    args.get(field)
        .map(String::as_str)
        .filter(|v| !v.trim().is_empty())
}

/// Option → Result: absence becomes an error with a reason, via ok_or.
fn required<'a>(args: &'a Args, field: &'static str) -> Result<&'a str, ArgError> {
    optional(args, field).ok_or(ArgError::Missing(field))
}

fn parse_number(field: &'static str, raw: &str) -> Result<f64, ArgError> {
    raw.trim().parse::<f64>().map_err(|_| ArgError::NotANumber {
        field,
        value: raw.to_string(),
    })
}

/// The `?` operator: on Err, return it from this function; on Ok, unwrap the value.
/// Python equivalent: letting an exception propagate, but visible in the signature.
fn parse_search(args: &Args) -> Result<SearchCall, ArgError> {
    let query = required(args, "query")?.to_string();

    // A default for an optional argument: Option + unwrap_or.
    let top_k = match optional(args, "top_k") {
        Some(raw) => parse_number("top_k", raw)?,
        None => 5.0,
    };
    if !(1.0..=50.0).contains(&top_k) || top_k.fract() != 0.0 {
        return Err(ArgError::OutOfRange {
            field: "top_k",
            value: top_k,
        });
    }

    // Option<&str> → Option<Result<f64>> → Result<Option<f64>> with transpose.
    let min_score = optional(args, "min_score")
        .map(|raw| parse_number("min_score", raw))
        .transpose()?;

    Ok(SearchCall {
        query,
        top_k: top_k as u32, // safe: checked to be a whole number in 1..=50 above
        min_score,
    })
}

fn args(pairs: &[(&str, &str)]) -> Args {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn main() {
    let cases = [
        args(&[("query", "rust ownership"), ("top_k", "3")]),
        args(&[("query", "borrowing"), ("min_score", "0.7")]),
        args(&[("top_k", "3")]),
        args(&[("query", "x"), ("top_k", "three")]),
        args(&[("query", "x"), ("top_k", "500")]),
        args(&[("query", "   ")]),
    ];
    for (i, case) in cases.iter().enumerate() {
        match parse_search(case) {
            Ok(call) => println!("case {i}: ok    {call:?}"),
            Err(e) => println!("case {i}: error {e}"),
        }
    }

    // A few Option combinators, side by side with Python.
    let scores = [0.42, 0.91, 0.77];
    let best = scores.iter().copied().reduce(f64::max); // max(scores, default=None)
    println!("best           : {best:?}");
    let first_high = scores.iter().find(|&&s| s > 0.8); // next((s for s in scores if s > 0.8), None)
    println!("first > 0.8    : {first_high:?}");
    let label = first_high
        .map(|s| format!("{:.0}%", s * 100.0))
        .unwrap_or_default();
    println!("map+default    : {label:?}");
    let none: Option<f64> = scores.iter().copied().find(|&s| s > 0.99);
    println!("unwrap_or(0.0) : {}", none.unwrap_or(0.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_top_k_to_five() {
        let call = parse_search(&args(&[("query", "q")])).unwrap();
        assert_eq!(call.top_k, 5);
        assert_eq!(call.min_score, None);
    }

    #[test]
    fn blank_query_counts_as_missing() {
        assert_eq!(
            parse_search(&args(&[("query", "  ")])),
            Err(ArgError::Missing("query"))
        );
    }

    #[test]
    fn rejects_non_numbers_and_fractions() {
        assert!(matches!(
            parse_search(&args(&[("query", "q"), ("top_k", "abc")])),
            Err(ArgError::NotANumber { field: "top_k", .. })
        ));
        assert!(matches!(
            parse_search(&args(&[("query", "q"), ("top_k", "2.5")])),
            Err(ArgError::OutOfRange { field: "top_k", .. })
        ));
    }

    #[test]
    fn bad_optional_value_is_an_error_not_ignored() {
        assert!(matches!(
            parse_search(&args(&[("query", "q"), ("min_score", "high")])),
            Err(ArgError::NotANumber {
                field: "min_score",
                ..
            })
        ));
    }
}
