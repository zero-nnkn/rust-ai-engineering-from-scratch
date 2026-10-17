//! End-to-end check on the sample file shipped with the lesson.

use project_word_stats::stats::compute;

#[test]
fn sample_file_statistics() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/samples/ownership.txt");
    let text = std::fs::read_to_string(path).expect("sample file is part of the repo");
    let s = compute(&text, 3, 4.0);
    assert_eq!(s.lines, 8);
    assert_eq!(s.words, 79);
    assert_eq!(s.top_words[0], ("owner".to_string(), 3));
    assert!(s.top_words.iter().any(|(w, _)| w == "ownership"));
}
