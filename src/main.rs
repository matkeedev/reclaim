//! reclaim - find and reclaim disk space from dev junk.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;

use reclaim::menu::{self, Choice};
use reclaim::scanner::{self, Hit, Scan};
use reclaim::tui::{self, Exit};
use reclaim::{cleaner, format};

/// Find and reclaim disk space from build & cache directories.
///
/// Run with no arguments for an interactive menu, or pass a path to scan
/// it directly.
#[derive(Parser)]
#[command(name = "reclaim", version, about)]
struct Cli {
    /// Directory to scan. Omit to open the interactive menu.
    path: Option<PathBuf>,

    /// List what would be cleaned, then exit. No UI, no deletion.
    #[arg(short, long)]
    list: bool,

    /// Delete everything found without the interactive UI.
    #[arg(short, long)]
    yes: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("reclaim: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();

    // Decide what to scan: an explicit path, the current dir (with flags),
    // or whatever the user picks from the menu.
    let path = match resolve_path(&cli) {
        Some(path) => path,
        None => return Ok(ExitCode::SUCCESS),
    };

    let scan = timed_scan(&path);

    if scan.hits.is_empty() {
        println!("nothing to reclaim - that tree is already clean");
        return Ok(ExitCode::SUCCESS);
    }

    if cli.list {
        print_list(&scan.hits);
        return Ok(ExitCode::SUCCESS);
    }

    if cli.yes {
        return Ok(report(&scan.hits));
    }

    match tui::run(scan.hits)? {
        Exit::Quit => Ok(ExitCode::SUCCESS),
        Exit::Clean(picked) => Ok(report(&picked)),
    }
}

/// Work out which directory to scan. `None` means "quit, do nothing".
fn resolve_path(cli: &Cli) -> Option<PathBuf> {
    if let Some(path) = &cli.path {
        return Some(path.clone());
    }

    // With a flag but no path, default to the current directory so the
    // tool stays scriptable (e.g. `reclaim --yes`).
    if cli.list || cli.yes {
        return Some(PathBuf::from("."));
    }

    match menu::choose() {
        Choice::Scan(path) => Some(path),
        Choice::Quit => None,
    }
}

/// Run a scan while timing it, then print a one-line summary.
fn timed_scan(path: &std::path::Path) -> Scan {
    eprintln!("scanning {} ...", path.display());

    let start = Instant::now();
    let scan = scanner::scan(path);
    let elapsed = start.elapsed();

    eprintln!(
        "walked {} dirs in {} - found {} reclaimable ({})",
        scan.dirs_scanned,
        format::duration(elapsed),
        scan.hits.len(),
        format::bytes(scan.total_size()),
    );

    scan
}

/// Print hits as a plain table, then a per-ecosystem summary.
fn print_list(hits: &[Hit]) {
    for hit in hits {
        println!(
            "{:>9}  {:<8} {}",
            format::bytes(hit.size),
            hit.kind,
            hit.path.display()
        );
    }

    println!();
    print_summary(hits);
}

/// Group hits by ecosystem and print a total for each.
fn print_summary(hits: &[Hit]) {
    let mut by_kind: BTreeMap<&str, (usize, u64)> = BTreeMap::new();
    for hit in hits {
        let entry = by_kind.entry(hit.kind).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += hit.size;
    }

    println!("by ecosystem:");
    for (kind, (count, size)) in &by_kind {
        println!("  {:<8} {:>9}  ({} dirs)", kind, format::bytes(*size), count);
    }

    let total: u64 = hits.iter().map(|h| h.size).sum();
    println!("\n{} reclaimable across {} dirs", format::bytes(total), hits.len());
}

/// Clean the given hits and print a summary.
fn report(hits: &[Hit]) -> ExitCode {
    let r = cleaner::clean(hits);
    println!("removed {} dirs, freed {}", r.removed, format::bytes(r.freed));

    if r.failed.is_empty() {
        return ExitCode::SUCCESS;
    }

    eprintln!("\n{} could not be removed:", r.failed.len());
    for (path, why) in &r.failed {
        eprintln!("  {path}: {why}");
    }
    ExitCode::FAILURE
}
