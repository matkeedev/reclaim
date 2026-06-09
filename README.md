# reclaim

> Find and reclaim disk space from dev junk — `node_modules`, `target/`, `__pycache__` and friends.

A fast, interactive terminal UI that walks your projects, tallies up the
disposable build & cache directories eating your disk, and lets you wipe them
in one keystroke. Written in Rust, scans in parallel, ships as a single binary.

```
  reclaim    14 dirs found   6.8 GB selected (11 dirs)  /  7.2 GB total
 ----------------------------------------------------------------------
 > [x]   3.1 GB  node     ~/work/dashboard/node_modules
   [x]   1.9 GB  rust     ~/work/engine/target
   [x]   820 MB  next     ~/work/site/.next
   [ ]   612 MB  node     ~/play/throwaway/node_modules
   [x]   140 MB  python   ~/ml/notebooks/.venv
 ----------------------------------------------------------------------
  up/dn move   space toggle   a all/none   enter clean   q quit
```

## Why

Build and dependency folders pile up silently. A laptop with a dozen side
projects can lose tens of gigabytes to stale `node_modules` and `target/`
directories you forgot about. `reclaim` finds them all, shows you the damage,
and only deletes what you confirm.

Unlike single-ecosystem cleaners, `reclaim` understands many stacks at once and
**only** flags a directory when it's genuinely disposable — a `node_modules` is
ignored unless a `package.json` sits beside it, so it never touches a folder
that merely shares the name.

## Install

From source (requires a Rust toolchain):

```sh
cargo install --path .
```

Or build a release binary directly:

```sh
cargo build --release
# binary lands at target/release/reclaim
```

## Usage

Run with no arguments to open the interactive menu:

```sh
reclaim
```

```
  reclaim - reclaim disk space from dev junk
  ------------------------------------------
  1) scan everything   (/home/you)
  2) scan current folder
  3) scan a specific folder
  q) quit
```

Pick `1` to sweep your whole home directory at once, `2` for the current
folder, or `3` to type a path. Every scan prints how long it took and how
much it walked, e.g. `walked 18204 dirs in 2.4 s - found 14 reclaimable (7.2 GB)`.

You can also skip the menu and scan a path directly:

```sh
reclaim ~/work
```

Just show what *would* be cleaned, no UI and no deletion (prints a per-ecosystem
summary):

```sh
reclaim --list ~/work
```

Delete everything found, no prompts (handy in scripts / CI):

```sh
reclaim --yes ~/work
```

### Keys

| Key        | Action            |
| ---------- | ----------------- |
| `↑` / `↓`  | move cursor       |
| `j` / `k`  | move cursor (vim) |
| `space`    | toggle one        |
| `a`        | select all / none |
| `enter`    | clean selected    |
| `q` / `esc`| quit, change nothing |

## What it cleans

| Directory                              | Stack        | Requires sibling |
| -------------------------------------- | ------------ | ---------------- |
| `node_modules`, `.next`, `.nuxt`       | JS / TS      | `package.json`   |
| `dist`, `build`                        | JS build     | `package.json`   |
| `target`                               | Rust         | `Cargo.toml`     |
| `__pycache__`, `.pytest_cache`, `.venv`| Python       | —                |
| `.mypy_cache`, `venv`                  | Python       | —                |
| `.gradle`                              | Java/Gradle  | —                |
| `vendor`                               | PHP / Go     | `composer.json`  |
| `.terraform`                           | Terraform    | —                |

Matched directories are never descended into, so a nested `node_modules` is
counted once and removed as a whole.

## Safety

- Nothing is deleted until you press `enter` (or pass `--yes`).
- A target only counts when its marker file is present, so source folders that
  happen to be named `build` or `dist` without a `package.json` are left alone.
- Deletions that fail (permissions, races) are reported, not silently swallowed.

## Development

```sh
cargo test       # unit + integration tests
cargo run -- --list .
```

## License

MIT — see [LICENSE](LICENSE).
