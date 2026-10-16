//! The app never hotlinks third-party hosts at runtime: every asset is embedded
//! as a `data:` URI through the manifest + generator pipeline. This guard keeps
//! `src/` free of Wowhead / zamimg URLs (adapted from the imported v4.1 asset
//! plan, Task 1).

use std::fs;
use std::path::Path;

fn scan(dir: &Path, hits: &mut Vec<String>) {
    for entry in fs::read_dir(dir).expect("src readable") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            scan(&path, hits);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("source readable");
        if text.contains("wowhead.com") || text.contains("zamimg.com") {
            hits.push(path.to_string_lossy().into_owned());
        }
    }
}

#[test]
fn source_code_never_points_at_wowhead_or_zamimg() {
    let mut hits = Vec::new();
    scan(Path::new("src"), &mut hits);
    assert!(hits.is_empty(), "third-party URLs in src/: {hits:?}");
}
