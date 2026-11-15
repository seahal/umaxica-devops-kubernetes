# Repository Guidelines

## Project Structure & Module Organization
- Root: repo metadata (`README.md`, `LICENSE`, `.github/workflows/`).
- `src/`: Kubernetes manifests per workload (e.g., `ror.yaml`, `sample.yml`). Group related resources in the same file or use clear file pairs (e.g., `app-deploy.yaml`, `app-svc.yaml`).
- Branch and PR changes should touch only the manifests they affect.

## Build, Test, and Development Commands
- Diff proposed changes: `kubectl diff -f src/ -R` (shows live vs. local).
- Server dry-run: `kubectl apply -f src/ -R --dry-run=server` (validates against API).
- Apply a single manifest: `kubectl apply -f src/ror.yaml -n default`.
- Inspect result: `kubectl get deploy -n default` and `kubectl describe deploy/app-ror`.
- CI: `.github/workflows/` contains the pipeline; extend with validation as needed (e.g., `kubectl kustomize`, `kubeconform`).

## Coding Style & Naming Conventions
- YAML: 2-space indentation; lists start with `-` aligned under parent key.
- Filenames: use kebab-case; prefer `.yaml` (unify over time from mixed `.yml`).
- Resource names/labels: kebab-case; keep labels consistent (`app`, `run`, `env`). Example:
  labels: `{ app: eks-sample-linux-app }` and match selectors accordingly.
- Images: avoid `:latest`; pin a version or digest.
- Manifests must include explicit `metadata.namespace` when not `default`.

## Testing Guidelines
- Before PR: run `kubectl diff` and `--dry-run=server` on changed files.
- Optional schema check: use `kubeconform` or `kubeval` if available.
- For local sanity tests, use a disposable cluster (kind/minikube) and apply only the changed manifests.
- Keep changes small and reversible; document any required prerequisites (CRDs, namespaces).

## Evidence

Completed tests, validations, verifications, audits, security checks and
performance checks leave a short record in `evidence/` when retaining the result
is useful. Records describe work that was actually performed — never plans,
intentions, or unverified claims. A check that could not be completed is
recorded as such, with the reason and whatever was observed.

- `evidence/` is flat; no subdirectories.
- Only `.md` files.
- `YYYY-MM-DD-<topic>.md`, ISO date, lowercase hyphenated topic.
- No raw logs, screenshots, binaries, archives, dumps, generated reports or
  other large artifacts. Summarize them, and cite the commands, identifiers,
  hashes, measurements and excerpts that carry the result.
- Enforced by `sh scripts/check-evidence.sh` (CI).

## Commit & Pull Request Guidelines
- Commits: follow Conventional Commits (e.g., `feat: add ror deployment`, `fix: pin nginx image tag`, `ci: extend validation workflow`).
- PRs must include: summary of changes, affected files, `kubectl diff` output (trimmed), linked issue, risk/rollback notes, and screenshots/log snippets when relevant.
- Avoid unrelated formatting churn; keep diffs focused.

## Security & Configuration Tips
- Do not commit secrets; reference `Secret`/`ConfigMap` resources and document creation steps.
- Set resource `requests`/`limits`; avoid cluster-wide privileges unless necessary.
- Prefer immutable image digests for production and declare namespaces explicitly.

