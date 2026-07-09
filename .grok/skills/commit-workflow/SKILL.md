---
name: commit-workflow
description: >
  Prepare, stage, document, commit, and push edt-chess changes.
  Use when the user asks to run the "Commit Workflow", /commit-workflow, commit and push,
  or prepare the project for a commit.
---

# Commit Workflow

Follow these steps in order. Do not skip documentation updates.

## 1. Inspect state

```bash
git status
git diff --stat
git branch -vv
git log --oneline -5
```

Ask the user about anything ambiguous (scope of commit, secrets, whether to push). Do not guess intent for destructive git operations.

## 2. Executable / CLI help

If any binary entrypoint or CLI flags changed (`src/main.rs`, scripts with `--help`):

- Update `print_help()` / script `usage()` text
- Ensure `--help` and `--version` still work
- Update README usage if flags changed

## 3. `.gitignore`

Ensure build artifacts are ignored (`/target/`, `/dist/`, editor junk). Add missing patterns before staging.

## 4. Stage files

Stage source, configs, docs, packaging, workflows, and lockfile as appropriate:

```bash
git add -A
git status
```

Do **not** stage secrets, local env files, or `target/` binaries.

## 5. Memory / plan files

Update project continuity docs so the next session can resume:

- `PLAN.md` — done / pending / known issues / next steps
- Adjust `TODO.md` or design docs if present

Include: what was done this session, key decisions, known issues, next steps.

## 6. README

Review and update `README.md` for features, setup, build/install, usage, structure, version.

## 7. Quality gates (when code changed)

Prefer running before commit:

```bash
cargo test
cargo build --release
./target/release/edt-chess --help
```

Or `./scripts/build-and-install.sh --skip-clean` if packaging was also requested.

## 8. Commit

Write a conventional commit message:

- Subject ≤72 chars, imperative mood
- Body explains what/why when not obvious

```bash
git commit -m "$(cat <<'EOF'
type(scope): short summary

Longer explanation if needed.

EOF
)"
```

## 9. Push

```bash
git push
# or if no upstream:
git push -u origin HEAD
```

## 10. Report

Show commit hash, branch, remote status, and remaining uncommitted files (if any).
