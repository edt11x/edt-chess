#!/usr/bin/env bash
# Build and Install Workflow for edt-chess
#
# Full clean build, test, package installer, and optional local install.
#
# Usage:
#   ./scripts/build-and-install.sh              # test + release build + installer tarball
#   ./scripts/build-and-install.sh --install    # also install to ~/.local
#   ./scripts/build-and-install.sh --prefix DIR # install prefix (default: $HOME/.local)
#   ./scripts/build-and-install.sh --skip-tests # skip cargo test
#   ./scripts/build-and-install.sh --help

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

PREFIX="${PREFIX:-$HOME/.local}"
DO_INSTALL=0
SKIP_TESTS=0
SKIP_CLEAN=0

usage() {
  cat <<'EOF'
edt-chess — Build and Install Workflow

USAGE:
    ./scripts/build-and-install.sh [OPTIONS]

OPTIONS:
    --install           Install binary + desktop entry to PREFIX (default: ~/.local)
    --prefix DIR        Installation prefix (default: $HOME/.local)
    --skip-tests        Skip cargo test
    --skip-clean        Do not run cargo clean first
    -h, --help          Show this help

STEPS:
    1. cargo clean (unless --skip-clean)
    2. cargo test
    3. cargo build --release
    4. Build installer tarball under dist/
    5. Optionally install to PREFIX

EXAMPLES:
    ./scripts/build-and-install.sh
    ./scripts/build-and-install.sh --install
    PREFIX=/usr/local ./scripts/build-and-install.sh --install --prefix /usr/local
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --install) DO_INSTALL=1; shift ;;
    --prefix) PREFIX="$2"; shift 2 ;;
    --skip-tests) SKIP_TESTS=1; shift ;;
    --skip-clean) SKIP_CLEAN=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown option: $1" >&2; usage >&2; exit 2 ;;
  esac
done

VERSION="$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)"/\1/')"
ARCH="$(uname -m)"
DIST_DIR="$ROOT/dist"
STAGE_NAME="edt-chess-${VERSION}-linux-${ARCH}"
STAGE="$DIST_DIR/$STAGE_NAME"
TARBALL="$DIST_DIR/${STAGE_NAME}.tar.gz"

echo "==> edt-chess build-and-install (version ${VERSION})"

if [[ "$SKIP_CLEAN" -eq 0 ]]; then
  echo "==> cargo clean"
  cargo clean
fi

if [[ "$SKIP_TESTS" -eq 0 ]]; then
  echo "==> cargo test"
  cargo test
fi

echo "==> cargo build --release"
cargo build --release

BIN="$ROOT/target/release/edt-chess"
if [[ ! -x "$BIN" ]]; then
  echo "error: release binary not found at $BIN" >&2
  exit 1
fi

echo "==> package installer → $TARBALL"
rm -rf "$STAGE"
mkdir -p "$STAGE/bin" "$STAGE/share/applications" "$STAGE/share/doc/edt-chess"
cp "$BIN" "$STAGE/bin/edt-chess"
cp "$ROOT/packaging/edt-chess.desktop" "$STAGE/share/applications/"
cp "$ROOT/README.md" "$STAGE/share/doc/edt-chess/"
cp "$ROOT/scripts/install.sh" "$STAGE/install.sh"
chmod +x "$STAGE/bin/edt-chess" "$STAGE/install.sh"

# Embed default prefix into staged install script helper note
cat > "$STAGE/README-INSTALL.txt" <<EOF
edt-chess ${VERSION} — Linux installer package

Quick install (user-local):
  ./install.sh
  # or: ./install.sh --prefix "\$HOME/.local"

System-wide (needs write access):
  sudo ./install.sh --prefix /usr/local

Then ensure PREFIX/bin is on PATH, and run:
  edt-chess
  edt-chess --help
  edt-chess --version
EOF

mkdir -p "$DIST_DIR"
tar -C "$DIST_DIR" -czf "$TARBALL" "$STAGE_NAME"
echo "    created $TARBALL"
ls -lh "$TARBALL"

if [[ "$DO_INSTALL" -eq 1 ]]; then
  echo "==> install to $PREFIX"
  "$ROOT/scripts/install.sh" --prefix "$PREFIX" --from-release
fi

echo "==> done"
echo "    binary:   $BIN"
echo "    tarball:  $TARBALL"
if [[ "$DO_INSTALL" -eq 1 ]]; then
  echo "    installed prefix: $PREFIX"
fi
