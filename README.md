# edt-chess

Graphical **chess practice** application for **Linux**, written in **Rust** with a **[Slint](https://slint.dev/)** UI.

**Version 0.3.0** — play against a built-in AI, train openings, solve tactics, export PGN.

## Features

- Interactive 8×8 board (click piece → destination)
- **Promotion dialog** (queen / rook / bishop / knight)
- Play as White or Black (board flips for Black)
- AI with **iterative deepening**, **transposition table**, α-β, quiescence, PST
- Difficulties: Easy (2) / Medium (3) / Hard (5)
- Hint, Undo, live evaluation
- **SAN move list** and **PGN export** (`~/.local/share/edt-chess/`)
- **Board themes**: Blue, Wood, Green
- **Opening trainer** (Italian, Ruy Lopez, Sicilian, Queen’s Gambit, London)
- **Tactics puzzles** (mate-in-1 and material)
- CLI: `--help`, `--version`

## Requirements

- Rust 1.75+
- Linux desktop (X11 or Wayland)

```bash
# Fedora
sudo dnf install gcc fontconfig-devel libxkbcommon-devel

# Debian/Ubuntu
sudo apt install build-essential libfontconfig1-dev libxkbcommon-dev
```

## Quick start

```bash
cargo run --release
edt-chess --help
```

## Build and Install Workflow

```bash
./scripts/build-and-install.sh           # clean, test, release, tarball
./scripts/build-and-install.sh --install # also install to ~/.local
make package
make install
```

Installer tarball: `dist/edt-chess-<version>-linux-<arch>.tar.gz`  
See [packaging/README.md](packaging/README.md) for Flatpak draft and future .deb/AppImage notes.

## Tests & CI

```bash
cargo test
```

GitHub Actions: [`.github/workflows/ci.yml`](.github/workflows/ci.yml) runs tests and a release build on Linux.

## Project layout

```
src/
  lib.rs game.rs ai.rs practice.rs   # library
  main.rs                            # GUI + CLI
ui/app-window.slint
tests/integration.rs
scripts/build-and-install.sh install.sh
packaging/
.github/workflows/ci.yml
.grok/skills/                        # agent workflows
PLAN.md AGENTS.md
```

## Agent workflows

- **build-and-install** — full clean build, package, optional install
- **commit-workflow** — stage, docs, commit, push

## Continuity

See [PLAN.md](PLAN.md) for status and next ideas.

## License

MIT
