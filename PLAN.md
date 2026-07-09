# edt-chess Plan

## Current status (2026-07-09)

**Branch:** `slint`  
**Version:** `0.2.0`  
**Stack:** Rust + Slint GUI + shakmaty rules engine

### Done this session

- Converted the Python terminal practice app into a native Linux GUI (Rust/Slint).
- Ported game logic (`ChessGame`) and AI (negamax, α-β, quiescence, PST, MVV-LVA).
- Interactive board: click-to-move, color choice, board flip, difficulty, hint, undo, eval.
- Refactored into `lib` + `bin` for testability; shared helpers (`color_name`, PST cache).
- Expanded unit + integration tests (39 total); mate-in-one AI check; fools/scholar mate paths.
- Fixed castling for UCI + GUI clicks (king landing square vs shakmaty rook-square `to()`).
- Added CLI `--help` / `--version`.
- Added **Build and Install Workflow** (`scripts/build-and-install.sh`, `Makefile`, packaging).
- Added **Commit Workflow** skill under `.grok/skills/commit-workflow/`.
- Version bumped to **0.2.0** (first Slint desktop release).
- Full clean build + installer tarball + install to `~/.local` verified.

### Key decisions

- **shakmaty** for rules (not python-chess).
- **Slint 1.17** for UI; Unicode piece glyphs (no SVG assets yet).
- AI runs on a **background thread**; UI updated via `invoke_from_event_loop`.
- Promotion defaults to **queen** (no promotion dialog yet).
- Installer is a **portable tarball** + `install.sh` to `~/.local` (not distro packages yet).

### Known issues

- No piece SVG/theme assets; glyphs depend on font coverage.
- No under-promotion UI.
- No PGN save/load or game history panel beyond last-move text.
- Hard AI (depth 4) can feel slow on weak CPUs (search is single-threaded pure Rust).
- `rustfmt` / `clippy` not installed in the current build environment; lint via `RUSTFLAGS="-D warnings" cargo check` / Makefile `lint`.

## Next logical steps

1. **Promotion dialog** when a pawn reaches the last rank (Q/R/B/N).
2. **Move list / SAN history** in the side panel; optional PGN export.
3. **Stronger engine options**: iterative deepening, transposition table, or optional Stockfish backend.
4. **Piece artwork** (SVG/PNG) and light/dark board themes.
5. **CI** (GitHub Actions): `cargo test` + release build on Linux.
6. **Distro packaging** later: Flatpak / `.deb` / AppImage if needed.
7. Practice modes from the old plan: openings trainer, tactics puzzles (data-driven).

## Workflows

| Name | How to run |
|------|------------|
| Build and Install | `./scripts/build-and-install.sh` or `make package` / skill `build-and-install` |
| Commit | skill `commit-workflow` / ask agent to “run the Commit Workflow” |
