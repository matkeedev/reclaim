//! Walk the filesystem, find reclaimable directories, and size them.

use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::targets::{self, Target};

/// One reclaimable directory we found on disk.
#[derive(Debug, Clone)]
pub struct Hit {
    pub path: PathBuf,
    pub kind: &'static str,
    pub size: u64,
    /// Whether the user has marked it for deletion.
    pub picked: bool,
}

/// The outcome of a scan: the hits plus a few stats for the UI.
#[derive(Debug)]
pub struct Scan {
    /// Reclaimable directories, sorted largest first.
    pub hits: Vec<Hit>,
    /// How many directories were walked while searching.
    pub dirs_scanned: usize,
}

impl Scan {
    /// Total bytes across every hit found.
    pub fn total_size(&self) -> u64 {
        self.hits.iter().map(|h| h.size).sum()
    }
}

/// Find every reclaimable directory under `root`.
///
/// Matched directories are never descended into, so a `node_modules`
/// nested inside another is counted only once.
pub fn scan(root: &Path) -> Scan {
    let (found, dirs_scanned) = collect(root);

    let mut hits: Vec<Hit> = found
        .par_iter()
        .map(|(path, target)| Hit {
            path: path.clone(),
            kind: target.kind,
            size: dir_size(path),
            picked: true,
        })
        .collect();

    hits.sort_by_key(|h| std::cmp::Reverse(h.size));
    Scan { hits, dirs_scanned }
}

/// Walk the tree and gather (path, target) pairs without sizing them.
///
/// Returns the matches and the number of directories visited.
fn collect(root: &Path) -> (Vec<(PathBuf, Target)>, usize) {
    let mut out = Vec::new();
    let mut scanned = 0usize;
    let mut walker = WalkDir::new(root).into_iter();

    while let Some(entry) = walker.next() {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_dir() {
            continue;
        }
        scanned += 1;

        let name = entry.file_name().to_string_lossy();
        let Some(target) = targets::match_dir(&name) else {
            continue;
        };

        if !marker_ok(entry.path(), &target) {
            continue;
        }

        out.push((entry.path().to_path_buf(), target));
        walker.skip_current_dir();
    }

    (out, scanned)
}

/// Check the sibling marker file (e.g. `package.json`) is present.
fn marker_ok(dir: &Path, target: &Target) -> bool {
    let Some(marker) = target.marker else {
        return true;
    };
    let Some(parent) = dir.parent() else {
        return false;
    };
    parent.join(marker).exists()
}

/// Sum the size of every file under `path`, following no symlinks.
fn dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}
