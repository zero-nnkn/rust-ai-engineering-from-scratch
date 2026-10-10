//! Phase 2 · Lesson 01 — Ownership & move semantics
//!
//! Run:  cargo run -p ownership-and-move
//! Test: cargo test -p ownership-and-move
//!
//! The scenario: a tiny "document batch" pipeline, the kind of thing you
//! write in Python every week. We use it to see exactly when Rust moves,
//! borrows, and clones a value.

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub id: u32,
    pub text: String,
}

/// Takes ownership of the batch. After calling this, the caller can no
/// longer use `docs` — the vector now lives (and dies) inside this function.
pub fn consume_batch(docs: Vec<Document>) -> usize {
    docs.iter().map(|d| d.text.len()).sum()
    // `docs` is dropped here: its memory is freed, deterministically.
}

/// Borrows the batch immutably. The caller keeps ownership.
/// Python equivalent: any function that only *reads* its argument.
pub fn total_chars(docs: &[Document]) -> usize {
    docs.iter().map(|d| d.text.len()).sum()
}

/// Borrows the batch mutably. Exactly one `&mut` may exist at a time,
/// which is why Rust can rule out data races at compile time.
pub fn normalize(docs: &mut [Document]) {
    for d in docs.iter_mut() {
        d.text = d.text.trim().to_lowercase();
    }
}

/// Takes ownership and *gives it back* transformed. This "move in, move out"
/// style is common for pipelines and costs no copying of the strings.
pub fn dedup(mut docs: Vec<Document>) -> Vec<Document> {
    docs.sort_by(|a, b| a.text.cmp(&b.text));
    docs.dedup_by(|a, b| a.text == b.text);
    docs.sort_by_key(|d| d.id);
    docs
}

fn sample() -> Vec<Document> {
    vec![
        Document {
            id: 1,
            text: "  Rust is FAST ".into(),
        },
        Document {
            id: 2,
            text: "rust is fast".into(),
        },
        Document {
            id: 3,
            text: "Ownership is the core idea".into(),
        },
    ]
}

fn main() {
    // 1. Borrow: read without taking ownership.
    let mut docs = sample();
    println!(
        "[borrow]  total chars before normalize = {}",
        total_chars(&docs)
    );

    // 2. Mutable borrow: change in place.
    normalize(&mut docs);
    println!(
        "[&mut]    after normalize: {:?}",
        docs.iter().map(|d| &d.text).collect::<Vec<_>>()
    );

    // 3. Move in, move out: `docs` is moved into dedup and a new Vec comes back.
    let docs = dedup(docs);
    println!("[move]    after dedup: {} docs", docs.len());

    // 4. Clone when you genuinely need two independent copies.
    let backup = docs.clone();

    // 5. Move for good: `docs` is consumed.
    let n = consume_batch(docs);
    println!("[consume] consumed {n} chars");

    // Uncomment the next line to see the compiler stop you:
    // println!("{}", docs.len());   // error[E0382]: borrow of moved value: `docs`

    // `backup` is still ours, because clone made an independent copy.
    println!("[clone]   backup still has {} docs", backup.len());

    // Copy types (integers, floats, bool, char) are copied, not moved.
    let a: u32 = 42;
    let b = a;
    println!("[copy]    a = {a}, b = {b} (both usable: u32 is Copy)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_keeps_ownership() {
        let docs = sample();
        let before = total_chars(&docs);
        assert_eq!(total_chars(&docs), before, "borrowing twice is fine");
    }

    #[test]
    fn normalize_then_dedup_removes_duplicate() {
        let mut docs = sample();
        normalize(&mut docs);
        let docs = dedup(docs);
        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].id, 1);
        assert_eq!(docs[0].text, "rust is fast");
    }

    #[test]
    fn clone_is_independent() {
        let mut original = sample();
        let copy = original.clone();
        normalize(&mut original);
        assert_ne!(original, copy);
    }
}
