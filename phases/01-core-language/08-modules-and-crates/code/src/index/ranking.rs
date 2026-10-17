//! TF-IDF scoring. A private child of `index`: users of the crate can't call it,
//! but `index` can, and this module can see `index`'s `pub(crate)` fields.

use super::Index; // `super` = the parent module (index)

/// Sum of tf * idf over the query terms, for one document.
pub(super) fn score(index: &Index, doc_id: usize, terms: &[String]) -> f64 {
    let Some(counts) = index.term_counts.get(doc_id) else {
        return 0.0;
    };
    terms
        .iter()
        .map(|t| {
            let tf = f64::from(counts.get(t).copied().unwrap_or(0));
            tf * idf(index, t)
        })
        .sum()
}

fn idf(index: &Index, term: &str) -> f64 {
    let n = index.term_counts.len() as f64;
    let df = index
        .term_counts
        .iter()
        .filter(|c| c.contains_key(term))
        .count() as f64;
    // Smoothed so that a term in every document still counts a little.
    ((n + 1.0) / (df + 1.0)).ln() + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rare_terms_weigh_more() {
        let mut ix = Index::new();
        ix.add("rust rust common");
        ix.add("common");
        assert!(idf(&ix, "rust") > idf(&ix, "common"));
    }

    #[test]
    fn unknown_doc_scores_zero() {
        let ix = Index::new();
        assert_eq!(score(&ix, 3, &["x".to_string()]), 0.0);
    }
}
