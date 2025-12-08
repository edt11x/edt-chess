# edt-chess
Just a python playtoy for chess

## Virtual environment (venv)

This project uses a local virtual environment directory called `.venv`.

- Create and activate the venv and install dependencies:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
```

- Or run the helper script:

```bash
bash scripts/setup_venv.sh
```

- Run the app:

```bash
source .venv/bin/activate
python chess.py
```

Notes:
- `tkinter` (used by `chess.py`) is part of system Python on many platforms and may require an OS package (for Debian/Ubuntu: `sudo apt install python3-tk`).
- Edit `requirements.txt` to add any third-party packages used by your scripts (for example `chess-moves` or `python-chess`).

```
