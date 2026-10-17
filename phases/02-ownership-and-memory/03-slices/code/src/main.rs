//! Phase 02 · Lesson 03 — Slices & views (zero-copy text)
//!
//! Run:  cargo run -p slices
//! Test: cargo test -p slices
//!
//! Scenario: preparing data for a model without copying it: chunking a document
//! into overlapping windows of text, and splitting a batch of embeddings.

/// Split `text` into chunks of at most `max_chars` characters with `overlap`
/// characters shared between neighbours. Every chunk is a &str pointing into
/// `text`: no characters are copied.
fn chunk_text(text: &str, max_chars: usize, overlap: usize) -> Vec<&str> {
    assert!(
        overlap < max_chars,
        "overlap must be smaller than max_chars"
    );
    // Byte offset of every character boundary, plus the end of the string.
    let bounds: Vec<usize> = text
        .char_indices()
        .map(|(i, _)| i)
        .chain(std::iter::once(text.len()))
        .collect();
    let n_chars = bounds.len() - 1;
    let step = max_chars - overlap;

    let mut chunks = Vec::new();
    let mut start = 0;
    while start < n_chars {
        let end = (start + max_chars).min(n_chars);
        chunks.push(&text[bounds[start]..bounds[end]]);
        if end == n_chars {
            break;
        }
        start += step;
    }
    chunks
}

/// The index of the first slice element that is >= target, in a sorted slice.
fn lower_bound(sorted: &[u32], target: u32) -> usize {
    sorted.partition_point(|&x| x < target)
}

/// Mean of each fixed-size row of a flat buffer (how embeddings are often stored:
/// one long Vec<f32>, `dim` values per row).
fn row_means(flat: &[f32], dim: usize) -> Vec<f32> {
    flat.chunks_exact(dim)
        .map(|row| row.iter().sum::<f32>() / dim as f32)
        .collect()
}

/// Moving average with a window, using `windows` (overlapping views).
fn moving_average(values: &[f64], window: usize) -> Vec<f64> {
    values
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect()
}

fn main() {
    let doc = "Slices are views. A view borrows; it never copies.";
    let chunks = chunk_text(doc, 20, 5);
    for (i, c) in chunks.iter().enumerate() {
        println!("chunk {i}: {c:?}");
    }
    // Prove there's no copy: the chunk's pointer is inside the original string.
    let start = doc.as_ptr() as usize;
    let offset = chunks[1].as_ptr() as usize - start;
    println!("chunk 1 starts at byte {offset} of the original");

    // Array slices: &v[a..b], first/last, split_at
    let token_ids: Vec<u32> = vec![101, 2023, 2003, 1037, 3231, 102];
    let body = &token_ids[1..token_ids.len() - 1]; // strip [CLS] and [SEP]
    println!("body          : {body:?}");
    let (head, tail) = body.split_at(2);
    println!("split_at(2)   : {head:?} | {tail:?}");
    println!("first/last    : {:?} {:?}", body.first(), body.last());

    // Two non-overlapping &mut views of one buffer, at the same time.
    let mut batch = vec![1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0];
    let (left, right) = batch.split_at_mut(3);
    left[0] = 10.0;
    right[0] = 40.0;
    println!("split_at_mut  : {batch:?}");

    println!("row means     : {:?}", row_means(&batch, 3));
    println!(
        "moving avg    : {:?}",
        moving_average(&[1.0, 2.0, 3.0, 4.0], 2)
    );

    let sorted = [10, 20, 20, 30, 40];
    println!("lower_bound 25: {}", lower_bound(&sorted, 25));
    println!("contains 30   : {}", sorted.contains(&30));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_overlap_and_cover_everything() {
        let chunks = chunk_text("abcdefghij", 4, 1);
        assert_eq!(chunks, ["abcd", "defg", "ghij"]);
    }

    #[test]
    fn chunking_is_utf8_safe() {
        let chunks = chunk_text("thế giới ơi", 3, 0);
        assert_eq!(chunks, ["thế", " gi", "ới ", "ơi"]);
    }

    #[test]
    fn short_text_is_one_chunk_and_empty_is_none() {
        assert_eq!(chunk_text("hi", 10, 2), ["hi"]);
        assert!(chunk_text("", 10, 2).is_empty());
    }

    #[test]
    fn row_means_ignores_incomplete_last_row() {
        assert_eq!(row_means(&[1.0, 3.0, 5.0, 7.0, 9.0], 2), [2.0, 6.0]);
    }

    #[test]
    fn lower_bound_finds_insertion_point() {
        assert_eq!(lower_bound(&[1, 3, 5], 0), 0);
        assert_eq!(lower_bound(&[1, 3, 5], 4), 2);
        assert_eq!(lower_bound(&[1, 3, 5], 9), 3);
    }
}
