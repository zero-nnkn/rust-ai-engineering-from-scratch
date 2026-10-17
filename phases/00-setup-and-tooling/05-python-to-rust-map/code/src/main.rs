//! Phase 00 · Lesson 05 — Python → Rust mental map
//!
//! Run:  cargo run -p python-to-rust-map
//! Test: cargo test -p python-to-rust-map
//!
//! Each section shows the Python you'd write (in a comment) and its Rust
//! equivalent. Don't try to understand every detail yet: Phases 01–03 cover
//! each one. Today the goal is to recognise the shapes.

use std::collections::HashMap;
use std::fmt;

// ---- dataclass → struct + derive -------------------------------------------
// @dataclass
// class Doc:
//     id: int
//     text: str
//     score: float = 0.0
#[derive(Debug, Clone, PartialEq)]
struct Doc {
    id: u32,
    text: String,
    score: f64,
}

// def __str__(self): ...      → impl Display
impl fmt::Display for Doc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Doc#{} ({:.2}): {}", self.id, self.score, self.text)
    }
}

// ---- Enum with data (no clean Python equivalent) ---------------------------
// role: Literal["user", "assistant", "tool"] plus different fields per role
#[derive(Debug)]
enum Message {
    User(String),
    Assistant { text: String, tokens: u32 },
    Tool { name: String, ok: bool },
}

// ---- try/except → Result + ? -----------------------------------------------
// def parse_score(s: str) -> float:
//     try:
//         v = float(s)
//     except ValueError:
//         raise ValueError(f"not a number: {s}")
//     if not 0 <= v <= 1:
//         raise ValueError(f"out of range: {v}")
//     return v
fn parse_score(s: &str) -> Result<f64, String> {
    let v: f64 = s.trim().parse().map_err(|_| format!("not a number: {s}"))?;
    if !(0.0..=1.0).contains(&v) {
        return Err(format!("out of range: {v}"));
    }
    Ok(v)
}

// ---- Optional[...] → Option ------------------------------------------------
// def best(docs) -> Optional[Doc]: return max(docs, key=..., default=None)
fn best(docs: &[Doc]) -> Option<&Doc> {
    docs.iter().max_by(|a, b| a.score.total_cmp(&b.score))
}

fn main() {
    let docs = vec![
        Doc {
            id: 1,
            text: "rust is fast".into(),
            score: 0.81,
        },
        Doc {
            id: 2,
            text: "python is friendly".into(),
            score: 0.64,
        },
        Doc {
            id: 3,
            text: "rust has no gc".into(),
            score: 0.92,
        },
    ];

    // list comprehension → iterator chain
    // [d.text.upper() for d in docs if d.score > 0.7]
    let high: Vec<String> = docs
        .iter()
        .filter(|d| d.score > 0.7)
        .map(|d| d.text.to_uppercase())
        .collect();
    println!("comprehension : {high:?}");

    // dict + Counter → HashMap + entry API
    // counts = Counter(w for d in docs for w in d.text.split())
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in docs.iter().flat_map(|d| d.text.split_whitespace()) {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!(
        "counts['rust']: {}",
        counts.get("rust").copied().unwrap_or(0)
    );

    // enumerate / zip / sum
    for (i, d) in docs.iter().enumerate().take(1) {
        println!("enumerate     : {i} {d}");
    }
    let total: f64 = docs.iter().map(|d| d.score).sum();
    println!("sum           : {total:.2}");

    // Optional
    match best(&docs) {
        Some(d) => println!("best          : {d}"),
        None => println!("best          : none"),
    }

    // try/except
    for raw in ["0.5", "abc", "1.7"] {
        match parse_score(raw) {
            Ok(v) => println!("parse {raw:>5} : ok {v}"),
            Err(e) => println!("parse {raw:>5} : error: {e}"),
        }
    }

    // match on an enum: like `match`/`case` in Python 3.10, but checked for completeness
    let msgs = vec![
        Message::User("hi".into()),
        Message::Assistant {
            text: "hello".into(),
            tokens: 3,
        },
        Message::Tool {
            name: "search".into(),
            ok: false,
        },
    ];
    for m in &msgs {
        let line = match m {
            Message::User(t) => format!("user said {t:?}"),
            Message::Assistant { text, tokens } => {
                format!("assistant said {text:?} ({tokens} tokens)")
            }
            Message::Tool { name, ok: false } => format!("tool {name} FAILED"),
            Message::Tool { name, ok: true } => format!("tool {name} ok"),
        };
        println!("match         : {line}");
    }

    // with open(...) as f:  → scope + Drop (the file closes when `file` goes out of scope)
    {
        let path = std::env::temp_dir().join("python-to-rust-map.txt");
        let written = std::fs::write(&path, "closed automatically\n").is_ok();
        println!("with-block    : wrote file = {written}");
        // Best-effort cleanup of a temp file; failure here doesn't matter.
        let _ = std::fs::remove_file(&path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_score_accepts_range() {
        assert_eq!(parse_score(" 0.25 "), Ok(0.25));
    }

    #[test]
    fn parse_score_rejects_text_and_range() {
        assert!(parse_score("abc").unwrap_err().contains("not a number"));
        assert!(parse_score("2").unwrap_err().contains("out of range"));
    }

    #[test]
    fn best_of_empty_is_none() {
        assert_eq!(best(&[]), None);
    }
}
