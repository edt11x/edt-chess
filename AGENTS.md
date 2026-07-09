# Agent notes for edt-chess

## Project

Linux chess practice GUI: **Rust + Slint + shakmaty**. Branch of interest: `slint`. Current version: see `Cargo.toml` (0.3.x feature set includes promotion, PGN, themes, openings, tactics, CI).

## Canonical workflows

1. **Build and Install Workflow**  
   Run `./scripts/build-and-install.sh` or skill `build-and-install`.  
   Produces `target/release/edt-chess` and `dist/*.tar.gz`.

2. **Commit Workflow**  
   Skill `commit-workflow`: inspect → help text → gitignore → stage → PLAN/README → test → commit → push.

## Conventions

- Keep game/AI logic in `src/game.rs` / `src/ai.rs` (library); keep Slint wiring in `src/main.rs`.
- Prefer tests under `src/*/tests` and `tests/integration.rs`.
- Bump `version` in `Cargo.toml` when releasing user-visible changes.
- Do not commit `/target/` or `/dist/`.

## Continuity

Session status lives in [PLAN.md](PLAN.md). Update it when finishing meaningful work.
