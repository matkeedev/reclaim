# reclaim

clean up the junk eating your disk — `node_modules`, `target/`, `__pycache__` and friends.
one keystroke, gone.

<!-- hero shot: the tui open with a full list of found dirs -->
![reclaim in action](https://i.imgur.com/Eke1xLF.png)

## why

i kept losing gigs to old `node_modules` and `target/` folders from projects i
hadn't touched in months. existing tools only cleaned one ecosystem at a time,
so i wrote my own. it scans everything, shows you the damage, and only deletes
what you tick.

written in rust, scans in parallel, ships as a single binary. no telemetry, no
network, nothing phoning home.

## install

needs a rust toolchain ([rustup.rs](https://rustup.rs)).

```sh
cargo install --path .
```

or just build it:

```sh
cargo build --release
# binary lands at target/release/reclaim
```

## usage

run it with nothing and you get a menu:

```sh
reclaim
```

<!-- screenshot of the menu (options 1/2/3/q) -->
![the menu](https://i.imgur.com/Eke1xLF.png)

- `1` sweeps your whole home folder at once
- `2` scans the current folder
- `3` lets you type a path

every scan tells you how long it took:

```
walked 18204 dirs in 2.4 s — found 14 reclaimable (7.2 GB)
```

prefer to skip the menu? point it straight at a path:

```sh
reclaim ~/work
```

just want to see what's there, no deleting? `--list` prints a breakdown:

```sh
reclaim --list ~/work
```

<!-- screenshot of --list output with the by-ecosystem summary -->
![list output](https://i.imgur.com/okkkZG5.png)

and for scripts, `--yes` nukes everything it finds, no prompts:

```sh
reclaim --yes ~/work
```

### keys

| key         | does           |
| ----------- | -------------- |
| `up` / `dn` | move           |
| `j` / `k`   | move (vim)     |
| `space`     | toggle one     |
| `a`         | all / none     |
| `enter`     | clean selected |
| `q`         | quit, no harm  |

## what it cleans

| folder                                  | stack        | needs neighbour |
| --------------------------------------- | ------------ | --------------- |
| `node_modules`, `.next`, `.nuxt`        | js / ts      | `package.json`  |
| `dist`, `build`                         | js build     | `package.json`  |
| `target`                                | rust         | `Cargo.toml`    |
| `__pycache__`, `.pytest_cache`, `.venv` | python       | —               |
| `.mypy_cache`, `venv`                   | python       | —               |
| `.gradle`                               | java/gradle  | —               |
| `vendor`                                | php / go     | `composer.json` |
| `.terraform`                            | terraform    | —               |

a folder only counts when its marker file sits next to it, so a `node_modules`
with no `package.json` beside it is left alone. your source code is never
touched.

## safety

nothing gets deleted until you hit `enter` (or pass `--yes`). `q` always leaves
without touching a thing. when in doubt, run `--list` first and look before you
wipe.

## license

mit — see [LICENSE](LICENSE).
