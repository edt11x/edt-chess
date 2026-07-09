# Packaging edt-chess

## Portable tarball (primary installer)

Produced by the **Build and Install Workflow**:

```bash
./scripts/build-and-install.sh
# → dist/edt-chess-<version>-linux-<arch>.tar.gz
```

Install from the tarball:

```bash
tar -xzf dist/edt-chess-*.tar.gz
cd edt-chess-*-linux-*
./install.sh                  # → ~/.local
# sudo ./install.sh --prefix /usr/local
```

## Desktop entry

`packaging/edt-chess.desktop` is included in the tarball and copied by `install.sh`.

## Flatpak (experimental)

See `packaging/flatpak/com.github.edt11x.edt_chess.yml`. Requires `flatpak-builder` and the Freedesktop SDK with the Rust extension. Not CI-gated yet.

## AppImage / .deb

Not automated. Recommended next steps:

- **AppImage**: wrap the release binary + desktop file with [linuxdeploy](https://github.com/linuxdeploy/linuxdeploy).
- **.deb**: use `cargo-deb` (`cargo install cargo-deb && cargo deb`) once a `[package.metadata.deb]` section is added to `Cargo.toml`.

## CI artifacts

CI workflow template: `packaging/github-actions-ci.yml`.

```bash
mkdir -p .github/workflows
cp packaging/github-actions-ci.yml .github/workflows/ci.yml
# commit & push with a token that has the `workflow` scope
```

When enabled, the workflow uploads `target/release/edt-chess` as an artifact.
