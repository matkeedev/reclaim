//! Actually remove the directories the user picked.

use std::fs;
use std::path::Path;

use crate::scanner::Hit;

/// Outcome of a clean run.
pub struct Report {
    pub removed: usize,
    pub freed: u64,
    pub failed: Vec<(String, String)>,
}

/// Delete every picked hit. Failures are collected, not fatal.
pub fn clean(hits: &[Hit]) -> Report {
    let mut report = Report {
        removed: 0,
        freed: 0,
        failed: Vec::new(),
    };

    for hit in hits.iter().filter(|h| h.picked) {
        match remove(&hit.path) {
            Ok(()) => {
                report.removed += 1;
                report.freed += hit.size;
            }
            Err(e) => {
                let where_ = hit.path.display().to_string();
                report.failed.push((where_, e.to_string()));
            }
        }
    }

    report
}

fn remove(path: &Path) -> std::io::Result<()> {
    fs::remove_dir_all(path)
}
