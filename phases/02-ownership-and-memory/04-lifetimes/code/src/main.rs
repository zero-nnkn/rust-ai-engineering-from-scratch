//! Phase 02 · Lesson 04 — Lifetimes without fear
//!
//! Run:  cargo run -p lifetimes
//! Test: cargo test -p lifetimes
//!
//! Scenario: a prompt template like "Answer {question} using {context}". We parse
//! it into pieces that *borrow* from the template string, so parsing copies nothing.
//! Lifetimes are how we tell the compiler "these pieces point into that template".

use std::collections::HashMap;

/// One piece of a template. Both variants hold a &str that points into the
/// template text. `'a` names "the lifetime of the template".
#[derive(Debug, PartialEq)]
enum Piece<'a> {
    Text(&'a str),
    Var(&'a str),
}

/// A parsed template. It can't outlive the string it was parsed from,
/// and the compiler checks that for us.
#[derive(Debug)]
struct Template<'a> {
    source: &'a str,
    pieces: Vec<Piece<'a>>,
}

#[derive(Debug, PartialEq)]
enum RenderError<'a> {
    MissingVar(&'a str),
}

impl<'a> Template<'a> {
    /// Elision would also work here, but writing 'a makes the link explicit:
    /// the pieces borrow from `source`.
    fn parse(source: &'a str) -> Template<'a> {
        let mut pieces = Vec::new();
        let mut rest = source;
        while let Some(open) = rest.find('{') {
            let Some(close) = rest[open..].find('}') else {
                break; // no closing brace: treat the rest as text
            };
            if open > 0 {
                pieces.push(Piece::Text(&rest[..open]));
            }
            pieces.push(Piece::Var(&rest[open + 1..open + close]));
            rest = &rest[open + close + 1..];
        }
        if !rest.is_empty() {
            pieces.push(Piece::Text(rest));
        }
        Template { source, pieces }
    }

    /// Lifetime elision: one input reference (&self) → the output borrows from it.
    /// Written out in full, this is `fn vars<'s>(&'s self) -> Vec<&'s str>`, but the
    /// returned &str actually point into the template text, so we can promise 'a.
    fn vars(&self) -> Vec<&'a str> {
        self.pieces
            .iter()
            .filter_map(|p| match p {
                Piece::Var(name) => Some(*name),
                Piece::Text(_) => None,
            })
            .collect()
    }

    /// The result is a new String (owned), so it has no lifetime tie to anything.
    /// The error borrows the variable name from the template: lifetime 'a again.
    fn render(&self, values: &HashMap<&str, String>) -> Result<String, RenderError<'a>> {
        let mut out = String::with_capacity(self.source.len());
        for piece in &self.pieces {
            match piece {
                Piece::Text(t) => out.push_str(t),
                Piece::Var(name) => match values.get(name) {
                    Some(v) => out.push_str(v),
                    None => return Err(RenderError::MissingVar(name)),
                },
            }
        }
        Ok(out)
    }
}

/// Two input references and a returned reference: the compiler can't guess
/// which input the output comes from, so we must say. 'a here means "the
/// result lives as long as the shorter-lived of the two inputs".
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// A &'static str lives for the whole program: string literals are baked into
/// the binary.
fn default_system_prompt() -> &'static str {
    "You are a helpful assistant."
}

fn main() {
    let template_text = String::from("Answer {question} using only {context}. Be brief.");
    let template = Template::parse(&template_text);
    println!("pieces : {:?}", template.pieces);
    println!("vars   : {:?}", template.vars());

    let mut values = HashMap::new();
    values.insert("question", "what is a lifetime?".to_string());
    values.insert("context", "the Rust book".to_string());
    match template.render(&values) {
        Ok(prompt) => println!("render : {prompt}"),
        Err(e) => println!("error  : {e:?}"),
    }

    values.remove("context");
    println!("missing: {:?}", template.render(&values));

    println!("longer : {}", longer("short", "a bit longer"));
    println!("static : {}", default_system_prompt());

    // The rendered prompt is owned, so it can outlive the template text:
    let rendered = {
        let temporary = String::from("Hi {name}!");
        let t = Template::parse(&temporary);
        let mut v = HashMap::new();
        v.insert("name", "Rustacean".to_string());
        t.render(&v).unwrap_or_default()
        // `temporary` and `t` are dropped here; `rendered` doesn't borrow them.
    };
    println!("owned  : {rendered}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_text_and_vars() {
        let t = Template::parse("a {x} b {y}");
        assert_eq!(
            t.pieces,
            [
                Piece::Text("a "),
                Piece::Var("x"),
                Piece::Text(" b "),
                Piece::Var("y")
            ]
        );
    }

    #[test]
    fn unclosed_brace_is_text() {
        let t = Template::parse("cost {oops");
        assert_eq!(t.pieces, [Piece::Text("cost {oops")]);
    }

    #[test]
    fn missing_variable_is_named() {
        let t = Template::parse("{a}{b}");
        let mut v = HashMap::new();
        v.insert("a", "1".to_string());
        assert_eq!(t.render(&v), Err(RenderError::MissingVar("b")));
    }

    #[test]
    fn pieces_point_into_the_source() {
        let src = String::from("x {y}");
        let t = Template::parse(&src);
        let Piece::Var(name) = t.pieces[1] else {
            panic!("expected a var");
        };
        let offset = name.as_ptr() as usize - src.as_ptr() as usize;
        assert_eq!(offset, 3);
    }
}
