//! Definitions of the "junk" directories we know how to reclaim.

/// A kind of disposable build/cache directory, tied to an ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target {
    /// Directory name to match, e.g. `node_modules`.
    pub dir: &'static str,
    /// Ecosystem label shown in the UI, e.g. `node`.
    pub kind: &'static str,
    /// File that must sit *next to* the dir for it to count.
    /// `None` means the dir name alone is enough.
    pub marker: Option<&'static str>,
}

/// Every directory pattern reclaim knows how to clean.
pub const TARGETS: &[Target] = &[
    Target {
        dir: "node_modules",
        kind: "node",
        marker: Some("package.json"),
    },
    Target {
        dir: "target",
        kind: "rust",
        marker: Some("Cargo.toml"),
    },
    Target {
        dir: ".next",
        kind: "next",
        marker: Some("package.json"),
    },
    Target {
        dir: ".nuxt",
        kind: "nuxt",
        marker: Some("package.json"),
    },
    Target {
        dir: "dist",
        kind: "build",
        marker: Some("package.json"),
    },
    Target {
        dir: "build",
        kind: "build",
        marker: Some("package.json"),
    },
    Target {
        dir: "__pycache__",
        kind: "python",
        marker: None,
    },
    Target {
        dir: ".pytest_cache",
        kind: "python",
        marker: None,
    },
    Target {
        dir: ".mypy_cache",
        kind: "python",
        marker: None,
    },
    Target {
        dir: ".gradle",
        kind: "java",
        marker: None,
    },
    Target {
        dir: ".venv",
        kind: "python",
        marker: None,
    },
    Target {
        dir: "venv",
        kind: "python",
        marker: None,
    },
    Target {
        dir: "vendor",
        kind: "php/go",
        marker: Some("composer.json"),
    },
    Target {
        dir: ".terraform",
        kind: "infra",
        marker: None,
    },
];

/// Return the matching target for a directory name, if any.
pub fn match_dir(name: &str) -> Option<Target> {
    TARGETS.iter().copied().find(|t| t.dir == name)
}

#[cfg(test)]
mod tests {
    use super::{match_dir, TARGETS};

    #[test]
    fn known_dirs_match() {
        assert!(match_dir("node_modules").is_some());
        assert!(match_dir("__pycache__").is_some());
        assert_eq!(match_dir("node_modules").unwrap().kind, "node");
    }

    #[test]
    fn unknown_dirs_do_not_match() {
        assert!(match_dir("src").is_none());
        assert!(match_dir("").is_none());
    }

    #[test]
    fn node_modules_requires_a_marker() {
        let t = match_dir("node_modules").unwrap();
        assert_eq!(t.marker, Some("package.json"));
    }

    #[test]
    fn target_table_is_non_empty() {
        assert!(!TARGETS.is_empty());
    }
}
