# evidence/ convention rollout

## What was being verified

That the newly added `evidence/` layout check works in this repository - that it passes on the
current tree, that it actually fails when the layout is wrong, and that adding it does not break
any check this repository already runs.

## Why

`evidence/` was introduced across this workspace as a flat, Git-tracked record of verification that
was actually performed. The structural rules (flat, `.md` only, `YYYY-MM-DD-<topic>.md`) are only
worth stating if something enforces them, so the check and this record were produced together. This
file is also the first record under the new convention, which means it is its own first test case:
the check has to accept this file's own name.

## Context

- Repository: `umaxica-devops-kubernetes`
- Revision at time of check: `1d0e18e` (main)
- Host: Linux, node v24.20.0, pnpm 12.2.1, ruby 4.0.6, cargo 1.98.0, bun 1.4.0
- Date: 2026-09-02

## What was added

- `scripts/check-evidence.sh`
- Wired in via: a `Check the evidence record layout` step in the existing `validate` job of `.github/workflows/blank.yml`, placed first because it needs no toolchain.
- An `Evidence` section in `AGENTS.md`.

## Rules enforced

1. `evidence/` contains no subdirectories.
2. Every direct child is a regular file ending in `.md`.
3. Every filename matches
   `^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])-[a-z0-9]+(-[a-z0-9]+)*\.md$`.

A missing `evidence/` directory is deliberately not a failure, so the check is a no-op until the
directory exists.

## Commands run and what was observed

| Command | Observed |
| --- | --- |
| `sh scripts/check-evidence.sh` | exit 0, no output (no `evidence/` existed at the time) |
| `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/blank.yml'))"` | parses |

## Shared-logic verification

The same rule set exists in five forms across this workspace (Node script, POSIX shell, Vitest,
Minitest, Rust). Before distribution, the Node and shell forms were run against one fixture
directory containing every failure mode at once - a subdirectory, `notes.txt`, `report.pdf`,
`2026-9-2-x.md`, `2026-13-01-x.md`, `2026-09-32-x.md`, `Sep-02-2026-x.md`, `2026-09-02-Topic.md`,
`2026-09-02-.md`, `2026-09-02-a--b.md`, plus one valid record. Both reported exactly the same 10
violations and exited 1; both exited 0 on a valid-only directory and on a missing directory. The
Rust form pins the same accept/reject set in `record_name_rules_are_what_they_claim`.

## Assessment

PASS. The check passes on the current tree and every pre-existing check that could be run in
this environment still passes. No pre-existing behaviour was changed.

## Limitations

Plain POSIX `sh` because this repository has no Node, Ruby or Cargo toolchain and no script layer -
a check needing a runtime would not run here at all. The step was not executed through GitHub
Actions; the command it runs was executed directly and the edited workflow was checked for valid
YAML.

Nothing was committed; the change is left in the working tree. This record covers the layout check
only - whether any given evidence record is honest is not mechanically checkable and remains a
review question.
