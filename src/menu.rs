//! The start menu shown when `reclaim` runs with no path and no flags.

use std::io::{self, Write};
use std::path::PathBuf;

use crate::paths;

/// What the user picked from the start menu.
pub enum Choice {
    /// Scan this directory (and everything under it).
    Scan(PathBuf),
    /// Leave without scanning.
    Quit,
}

/// Show the menu and loop until the user makes a valid choice.
pub fn choose() -> Choice {
    let home = paths::home();

    loop {
        render(&home);

        match prompt("  choose: ").trim() {
            "1" => return Choice::Scan(home),
            "2" => return Choice::Scan(PathBuf::from(".")),
            "3" => {
                let raw = prompt("  path to scan: ");
                let path = raw.trim();
                if !path.is_empty() {
                    return Choice::Scan(PathBuf::from(path));
                }
            }
            "q" | "Q" => return Choice::Quit,
            _ => println!("  ? please pick 1, 2, 3 or q"),
        }
    }
}

/// Draw the menu screen.
fn render(home: &std::path::Path) {
    println!();
    println!("  reclaim - reclaim disk space from dev junk");
    println!("  ------------------------------------------");
    println!("  1) scan everything   ({})", home.display());
    println!("  2) scan current folder");
    println!("  3) scan a specific folder");
    println!("  q) quit");
    println!();
}

/// Print a prompt, flush, and read one line from stdin.
fn prompt(label: &str) -> String {
    print!("{label}");
    io::stdout().flush().ok();

    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line
}
