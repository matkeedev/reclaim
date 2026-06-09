//! reclaim - find and reclaim disk space from dev junk.
//!
//! The crate is split into small focused modules:
//! [`targets`] knows *what* to look for, [`scanner`] finds and sizes it,
//! [`cleaner`] removes it, [`menu`] is the start screen, and [`tui`] is the
//! interactive selection front-end.

pub mod cleaner;
pub mod format;
pub mod menu;
pub mod paths;
pub mod scanner;
pub mod targets;
pub mod tui;
