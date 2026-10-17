//! Text statistics. Pure functions: text in, numbers out, no I/O.

use std::collections::HashMap;
use std::fmt;

const STOPWORDS: [&str; 12] = [
    "a", "an", "and", "the", "is", "of", "to", "it", "in", "at", "can", "be",
];

/// Anything that can produce statistics. (A first taste of traits: Phase 03.)
pub trait TextStats {
    fn stats(&self, top: usize, chars_per_token: f64) -> Stats;
}

impl TextStats for str {
    fn stats(&self, top: usize, chars_per_token: f64) -> Stats {
        compute(self, top, chars_per_token)
    }
}

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub bytes: usize,
    pub chars: usize,
    pub lines: usize,
    pub words: usize,
    pub unique_words: usize,
    pub est_tokens: usize,
    pub avg_word_len: f64,
    pub longest_word: Option<String>,
    pub top_words: Vec<(String, usize)>,
}

fn normalize(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'')
        .trim_matches('\'')
        .to_lowercase()
}

pub fn compute(text: &str, top: usize, chars_per_token: f64) -> Stats {
    let words: Vec<String> = text
        .split_whitespace()
        .map(normalize)
        .filter(|w| !w.is_empty())
        .collect();

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for w in &words {
        *counts.entry(w.as_str()).or_insert(0) += 1;
    }

    let mut top_words: Vec<(String, usize)> = counts
        .iter()
        .filter(|(w, _)| !STOPWORDS.contains(w))
        .map(|(w, &n)| (w.to_string(), n))
        .collect();
    // Most frequent first; ties alphabetically, so the output is deterministic.
    top_words.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    top_words.truncate(top);

    let chars = text.chars().count();
    let total_word_chars: usize = words.iter().map(|w| w.chars().count()).sum();
    let longest_word = words
        .iter()
        .max_by(|a, b| {
            a.chars()
                .count()
                .cmp(&b.chars().count())
                .then_with(|| b.cmp(a)) // on equal length, prefer the alphabetically first
        })
        .cloned();

    Stats {
        bytes: text.len(),
        chars,
        lines: text.lines().count(),
        words: words.len(),
        unique_words: counts.len(),
        est_tokens: (chars as f64 / chars_per_token).ceil() as usize,
        avg_word_len: if words.is_empty() {
            0.0
        } else {
            total_word_chars as f64 / words.len() as f64
        },
        longest_word,
        top_words,
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "bytes         {:>8}", self.bytes)?;
        writeln!(f, "characters    {:>8}", self.chars)?;
        writeln!(f, "lines         {:>8}", self.lines)?;
        writeln!(f, "words         {:>8}", self.words)?;
        writeln!(f, "unique words  {:>8}", self.unique_words)?;
        writeln!(f, "est. tokens   {:>8}", self.est_tokens)?;
        writeln!(f, "avg word len  {:>8.2}", self.avg_word_len)?;
        writeln!(
            f,
            "longest word  {:>8}",
            self.longest_word.as_deref().unwrap_or("-")
        )?;
        writeln!(f, "top words (stopwords excluded):")?;
        for (word, n) in &self.top_words {
            writeln!(f, "  {n:>4}  {word}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text() {
        let s = compute("", 5, 4.0);
        assert_eq!(s.words, 0);
        assert_eq!(s.avg_word_len, 0.0);
        assert_eq!(s.longest_word, None);
        assert!(s.top_words.is_empty());
    }

    #[test]
    fn counts_are_case_and_punctuation_insensitive() {
        let s = compute("Rust, rust! RUST? the the", 5, 4.0);
        assert_eq!(s.words, 5);
        assert_eq!(s.unique_words, 2);
        assert_eq!(s.top_words, vec![("rust".to_string(), 3)]);
    }

    #[test]
    fn apostrophes_inside_words_are_kept() {
        let s = compute("'Rust's' ownership", 5, 4.0);
        assert_eq!(s.longest_word.as_deref(), Some("ownership"));
        assert!(s.top_words.iter().any(|(w, _)| w == "rust's"));
    }

    #[test]
    fn utf8_counts_chars_not_bytes() {
        let s = compute("thế giới", 5, 4.0);
        assert_eq!(s.chars, 8);
        assert_eq!(s.bytes, 12);
        assert_eq!(s.est_tokens, 2);
    }

    #[test]
    fn trait_method_matches_function() {
        let text = "a b c";
        assert_eq!(text.stats(3, 4.0), compute(text, 3, 4.0));
    }
}
