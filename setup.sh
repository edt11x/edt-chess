#!/usr/bin/env bash
set -e

if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
    echo "Installing dependencies..."
    venv/bin/pip install --upgrade pip -q
    venv/bin/pip install -r requirements.txt
    echo "Done. Activate with: source venv/bin/activate"
else
    echo "venv already exists. Activate with: source venv/bin/activate"
fi
