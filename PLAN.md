# edt-chess Plan

## Current status (2026-07-09)

**Branch:** `slint`  
**Version:** `0.3.0`  
**Stack:** Rust + Slint + shakmaty

### Done (v0.3.0 feature pass)

| Area | Status |
|------|--------|
| Promotion dialog Q/R/B/N | Done |
| SAN move list + PGN export | Done |
| Stronger AI (ID + TT + depths to 5) | Done |
| Board themes (classic/wood/green) | Done |
| GitHub Actions CI | Template at `packaging/github-actions-ci.yml` (copy to `.github/workflows/` with workflow-scope token) |
| Packaging docs + Flatpak draft | Done |
| Opening trainer (5 lines) | Done |
| Tactics puzzles (4 positions) | Done |

### Earlier (v0.2.0)

- Full Python → Rust/Slint conversion
- Build and Install + Commit workflows
- Castling for UCI and GUI clicks

### Key decisions

- Pure-Rust AI (no Stockfish dependency yet); hard = depth 5 with TT
- PGN saved under `~/.local/share/edt-chess/`
- Practice modes share the same board UI and move entry path
- Unicode piece glyphs (theme changes board colors only)

### Known issues / limits

- No SVG piece set yet
- Opening trainer is “as White” only for book lines
- Tactics set is small and hand-authored
- Flatpak/AppImage/.deb not fully automated
- Hard AI still single-threaded; can feel slow on deep trees

## Next ideas (post-0.3.0)

1. SVG/PNG piece themes and piece-set picker  
2. Expand puzzle DB (JSON load from file)  
3. Opening trainer as Black + multi-variation trees  
4. Optional Stockfish UCI backend  
5. `cargo-deb` / linuxdeploy AppImage in CI  
6. Move clock / timed puzzles  
7. Sound effects and accessibility (screen-reader labels)

## Workflows

| Name | How |
|------|-----|
| Build and Install | `./scripts/build-and-install.sh` or skill `build-and-install` |
| Commit | skill `commit-workflow` |
