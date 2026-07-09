#!/usr/bin/env bash
# Install edt-chess binary and desktop entry.
#
# Usage:
#   ./scripts/install.sh [--prefix DIR] [--from-release]
#   ./install.sh [--prefix DIR]   # when run from a staged dist package

set -euo pipefail

PREFIX="${PREFIX:-$HOME/.local}"
FROM_RELEASE=0

usage() {
  cat <<'EOF'
edt-chess installer

USAGE:
    ./scripts/install.sh [OPTIONS]
    ./install.sh [OPTIONS]          # from packaged tarball root

OPTIONS:
    --prefix DIR      Install prefix (default: $HOME/.local)
    --from-release    Copy from target/release (repo checkout)
    -h, --help        Show this help

Installs:
    $PREFIX/bin/edt-chess
    $PREFIX/share/applications/edt-chess.desktop
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prefix) PREFIX="$2"; shift 2 ;;
    --from-release) FROM_RELEASE=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown option: $1" >&2; usage >&2; exit 2 ;;
  esac
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Detect whether we are inside a staged package (has bin/edt-chess next to us)
# or the repo scripts/ directory.
if [[ -x "$SCRIPT_DIR/bin/edt-chess" ]]; then
  # Staged package root
  SRC_BIN="$SCRIPT_DIR/bin/edt-chess"
  SRC_DESKTOP="$SCRIPT_DIR/share/applications/edt-chess.desktop"
elif [[ -x "$SCRIPT_DIR/../bin/edt-chess" ]]; then
  SRC_BIN="$SCRIPT_DIR/../bin/edt-chess"
  SRC_DESKTOP="$SCRIPT_DIR/../share/applications/edt-chess.desktop"
elif [[ "$FROM_RELEASE" -eq 1 ]]; then
  ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
  SRC_BIN="$ROOT/target/release/edt-chess"
  SRC_DESKTOP="$ROOT/packaging/edt-chess.desktop"
  if [[ ! -x "$SRC_BIN" ]]; then
    echo "error: release binary missing; run cargo build --release first" >&2
    exit 1
  fi
else
  ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
  SRC_BIN="$ROOT/target/release/edt-chess"
  SRC_DESKTOP="$ROOT/packaging/edt-chess.desktop"
  if [[ ! -x "$SRC_BIN" ]]; then
    echo "error: release binary missing; run ./scripts/build-and-install.sh first" >&2
    exit 1
  fi
fi

mkdir -p "$PREFIX/bin" "$PREFIX/share/applications"
install -m 755 "$SRC_BIN" "$PREFIX/bin/edt-chess"
if [[ -f "$SRC_DESKTOP" ]]; then
  # Rewrite Exec= to installed path when possible
  sed "s|^Exec=.*|Exec=$PREFIX/bin/edt-chess|" "$SRC_DESKTOP" \
    > "$PREFIX/share/applications/edt-chess.desktop"
fi

echo "Installed edt-chess to $PREFIX/bin/edt-chess"
if command -v "$PREFIX/bin/edt-chess" >/dev/null 2>&1 || [[ -x "$PREFIX/bin/edt-chess" ]]; then
  "$PREFIX/bin/edt-chess" --version || true
fi
echo "Ensure $PREFIX/bin is on your PATH."
