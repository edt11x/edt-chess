---
name: build-and-install
description: >
  Full clean build, test, package, and install workflow for edt-chess.
  Use when the user asks to "build and install", run the "build and install workflow",
  package a release tarball, or /build-and-install.
---

# Build and Install Workflow

Run the project's canonical build/install pipeline. Prefer the scripts over ad-hoc cargo commands.

## Steps

1. **Working directory**: ensure you are in the `edt-chess` repo root (or the `slint` worktree containing `Cargo.toml`).

2. **Execute**:
   ```bash
   chmod +x scripts/build-and-install.sh scripts/install.sh
   ./scripts/build-and-install.sh
   ```
   Optional flags:
   - `--install` — also install to `~/.local` (or `--prefix DIR`)
   - `--skip-tests` — skip `cargo test` (only if user requests)
   - `--skip-clean` — skip `cargo clean` for faster iteration

   Makefile equivalents:
   - `make package` / `make workflow`
   - `make install`

3. **Verify**:
   ```bash
   ./target/release/edt-chess --version
   ./target/release/edt-chess --help
   ls -lh dist/edt-chess-*-linux-*.tar.gz
   ```

4. **Report** to the user:
   - test pass/fail summary
   - path to release binary
   - path to installer tarball under `dist/`
   - whether install was performed and to which prefix

## Do not

- Do not push commits as part of this workflow (use **commit** workflow).
- Do not run the GUI unless the user asks (`edt-chess` opens a window).
- Do not skip tests unless the user explicitly allows it.
