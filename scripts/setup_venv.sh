#!/usr/bin/env bash
set -euo pipefail

# Create and activate a local venv named .venv and install requirements if present.
PYTHON=${PYTHON:-python3}
VENV_DIR=".venv"

if [ -d "$VENV_DIR" ]; then
  echo "Using existing venv in $VENV_DIR"
else
  echo "Creating venv in $VENV_DIR using $PYTHON"
  $PYTHON -m venv "$VENV_DIR"
fi

# shellcheck source=/dev/null
source "$VENV_DIR/bin/activate"

pip install --upgrade pip
if [ -f requirements.txt ] && [ -s requirements.txt ]; then
  echo "Installing from requirements.txt"
  pip install -r requirements.txt
else
  echo "No requirements.txt or file is empty — nothing to install"
fi

echo "Virtual environment ready. Activate with: source $VENV_DIR/bin/activate"
