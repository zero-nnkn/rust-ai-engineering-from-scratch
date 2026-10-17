//! Phase 02 · Lesson 02 — Borrowing: &T and &mut T
//!
//! Run:  cargo run -p borrowing
//! Test: cargo test -p borrowing
//!
//! Scenario: a small embedding store. Reading vectors (similarity search) needs
//! shared borrows; normalizing them in place needs an exclusive borrow.

#[derive(Debug)]
struct EmbeddingStore {
    ids: Vec<String>,
    vectors: Vec<Vec<f32>>,
}

/// Shared borrow (&): read-only, and any number of them can exist at once.
fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn norm(v: &[f32]) -> f32 {
    dot(v, v).sqrt()
}

/// Exclusive borrow (&mut): may modify, and while it exists nobody else may
/// read or write the same data.
fn normalize(v: &mut [f32]) {
    let n = norm(v); // reborrow: &mut [f32] can be lent out as & [f32] for a moment
    if n > 0.0 {
        for x in v.iter_mut() {
            *x /= n; // `*` dereferences: change the value behind the reference
        }
    }
}

impl EmbeddingStore {
    fn new() -> Self {
        EmbeddingStore {
            ids: Vec::new(),
            vectors: Vec::new(),
        }
    }

    // Takes ownership of `id` and `vector`: the store keeps them.
    fn add(&mut self, id: &str, vector: Vec<f32>) {
        self.ids.push(id.to_string());
        self.vectors.push(vector);
    }

    fn normalize_all(&mut self) {
        for v in self.vectors.iter_mut() {
            normalize(v);
        }
    }

    /// Returns a reference *into* the store: no copy of the id string is made.
    /// The caller can use it as long as the store isn't modified.
    fn most_similar(&self, query: &[f32]) -> Option<(&str, f32)> {
        self.ids
            .iter()
            .zip(&self.vectors)
            .map(|(id, v)| (id.as_str(), dot(query, v)))
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }
}

fn main() {
    let mut store = EmbeddingStore::new();
    store.add("rust", vec![3.0, 4.0, 0.0]);
    store.add("python", vec![0.0, 1.0, 1.0]);
    store.add("borrowing", vec![2.0, 2.0, 1.0]);

    // Several shared borrows at the same time: fine.
    let first = &store.vectors[0];
    let second = &store.vectors[1];
    println!("norms before  : {:.3} {:.3}", norm(first), norm(second));

    // An exclusive borrow. The shared borrows above are no longer used after
    // the previous line, so the compiler allows this (non-lexical lifetimes).
    store.normalize_all();
    println!("norm after    : {:.3}", norm(&store.vectors[0]));
    println!("rust vector   : {:?}", store.vectors[0]);

    let mut query = vec![1.0, 1.0, 0.0];
    normalize(&mut query);
    if let Some((id, score)) = store.most_similar(&query) {
        // `id` borrows from `store`. While `id` is alive, `store` can't be changed.
        println!("best match    : {id} ({score:.3})");
    }

    // A borrow can be used, then end, then a mutable borrow can start:
    let best_id = store.most_similar(&query).map(|(id, _)| id.to_string()); // copy out what you need to keep
    store.add("new", vec![1.0, 0.0, 0.0]); // fine: no borrow of `store` is alive
    println!(
        "kept copy     : {best_id:?}, store now has {} vectors",
        store.ids.len()
    );

    // Mutating through &mut in a function call, then reading again:
    let mut counter = 0;
    bump(&mut counter);
    bump(&mut counter);
    println!("counter       : {counter}");
}

fn bump(n: &mut i32) {
    *n += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_gives_unit_length() {
        let mut v = vec![3.0, 4.0];
        normalize(&mut v);
        assert!((norm(&v) - 1.0).abs() < 1e-6);
        assert_eq!(v, [0.6, 0.8]);
    }

    #[test]
    fn normalize_leaves_zero_vector_alone() {
        let mut v = vec![0.0, 0.0];
        normalize(&mut v);
        assert_eq!(v, [0.0, 0.0]);
    }

    #[test]
    fn most_similar_of_empty_store_is_none() {
        assert_eq!(EmbeddingStore::new().most_similar(&[1.0]), None);
    }
}
