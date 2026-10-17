//! The binary crate. It sees the library exactly like any other user would:
//! only `pub` items, through the paths the library chose to expose.

use modules_and_crates::text::words;
use modules_and_crates::{Index, SearchHit, VERSION};

fn main() {
    println!("modules-and-crates v{VERSION}");

    let mut index = Index::new();
    for doc in [
        "Rust modules are declared with mod.",
        "Python packages use __init__.py files.",
        "Visibility in Rust: pub, pub(crate), and private by default.",
        "Rust crates are libraries or binaries.",
    ] {
        index.add(doc);
    }

    println!("words  : {:?}", words("Private by DEFAULT!"));
    for query in ["rust visibility", "python packages", "java"] {
        let hits: Vec<SearchHit> = index.search(query, 2);
        println!("query  : {query:?}");
        for h in &hits {
            println!(
                "  {:.3}  #{} {}",
                h.score,
                h.doc_id,
                index.get(h.doc_id).unwrap_or("?")
            );
        }
        if hits.is_empty() {
            println!("  (no results)");
        }
    }
    // Uncomment either line to see privacy enforced across the crate boundary:
    // let _ = index.term_counts;                 // pub(crate): hidden from the binary
    // modules_and_crates::index::ranking::score; // private module
}
