# edt-chess

Graphical **chess practice** application for **Linux**, written in **Rust** with a **[Slint](https://slint.dev/)** UI.

Version **0.2.0** — play against a built-in AI (negamax + alpha-beta + quiescence, piece-square tables), get hints, undo moves, and choose difficulty.

## Features

- Interactive 8×8 board (click piece → click destination)
- Play as White or Black (board flips for Black)
- AI opponent with Easy / Medium / Hard search depths
- Hint (suggested move + evaluation)
- Undo (takes back your move and the AI reply)
- Position evaluation display
- Auto-queen promotion
- CLI: `--help`, `--version`

## Requirements

- Rust toolchain (1.75+ recommended)
- Linux desktop (X11 or Wayland)
- Typical build dependencies for Slint/winit:

```bash
# Fedora
sudo dnf install gcc fontconfig-devel libxkbcommon-devel

# Debian/Ubuntu
sudo apt install build-essential libfontconfig1-dev libxkbcommon-dev
```

## Quick start

```bash
cargo run --release
```

```bash
edt-chess --help
edt-chess --version
```

## Build and Install Workflow

Full clean build, tests, release binary, and installer tarball:

```bash
./scripts/build-and-install.sh
```

Install to `~/.local` (binary + desktop entry):

```bash
./scripts/build-and-install.sh --install
# or
make install
```

| Option | Meaning |
|--------|---------|
| `--install` | Install into `PREFIX` (default `~/.local`) |
| `--prefix DIR` | Installation prefix |
| `--skip-tests` | Skip `cargo test` |
| `--skip-clean` | Skip `cargo clean` |

Artifacts:

- Binary: `target/release/edt-chess`
- Installer tarball: `dist/edt-chess-<version>-linux-<arch>.tar.gz`

From the tarball:

```bash
tar -xzf dist/edt-chess-*-linux-*.tar.gz
cd edt-chess-*-linux-*
./install.sh
```

Makefile shortcuts: `make test`, `make release`, `make package`, `make install`, `make lint`.

## Tests

```bash
cargo test
```

Covers board rules, undo, mate sequences, AI legality, difficulty mapping, and integration play loops.

## Project layout

```
Cargo.toml
build.rs
Makefile
PLAN.md
src/
  lib.rs          # library root (version, shared helpers)
  main.rs         # GUI + CLI entrypoint
  game.rs         # Board state (shakmaty)
  ai.rs           # Search & evaluation
ui/
  app-window.slint
tests/
  integration.rs
scripts/
  build-and-install.sh   # Build and Install Workflow
  install.sh
packaging/
  edt-chess.desktop
.grok/skills/
  build-and-install/     # agent skill
  commit-workflow/       # agent skill
```

## Agent workflows

Project skills (Grok / compatible agents):

- **build-and-install** — clean build, test, package, optional install
- **commit-workflow** — stage, update docs/plan, commit, push

## Development notes

- Library crate `edt_chess` holds game/AI logic; binary is UI-only glue.
- AI search runs on a background thread so the window stays responsive.
- See [PLAN.md](PLAN.md) for status, decisions, and next steps.

## License

MIT (see `Cargo.toml`).
