//! End-to-end test: build a temporary tree and verify what gets found.

use std::fs;
use std::path::PathBuf;

use reclaim::scanner;

/// Build a throwaway directory tree and hand back its root.
fn fixture(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("reclaim-test-{name}"));
    let _ = fs::remove_dir_all(&root);

    // A valid node project.
    fs::create_dir_all(root.join("app/node_modules/dep")).unwrap();
    fs::write(root.join("app/package.json"), "{}").unwrap();
    fs::write(root.join("app/node_modules/dep/a.js"), vec![0u8; 4096]).unwrap();

    // A python cache (needs no marker).
    fs::create_dir_all(root.join("py/__pycache__")).unwrap();
    fs::write(root.join("py/__pycache__/x.pyc"), vec![0u8; 2048]).unwrap();

    // node_modules with no package.json sibling: must be ignored.
    fs::create_dir_all(root.join("orphan/node_modules")).unwrap();
    fs::write(root.join("orphan/node_modules/y.bin"), vec![0u8; 999]).unwrap();

    root
}

#[test]
fn finds_valid_targets_and_skips_orphans() {
    let root = fixture("basic");
    let scan = scanner::scan(&root);
    let hits = &scan.hits;

    let kinds: Vec<&str> = hits.iter().map(|h| h.kind).collect();
    assert!(kinds.contains(&"node"), "should find node_modules");
    assert!(kinds.contains(&"python"), "should find __pycache__");
    assert_eq!(hits.len(), 2, "orphan node_modules must be skipped");

    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn results_are_sorted_largest_first() {
    let root = fixture("sorted");
    let scan = scanner::scan(&root);
    let hits = &scan.hits;

    for pair in hits.windows(2) {
        assert!(pair[0].size >= pair[1].size, "hits must be size-descending");
    }

    fs::remove_dir_all(&root).unwrap();
}
