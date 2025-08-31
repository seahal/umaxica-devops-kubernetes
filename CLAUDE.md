# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Kubernetes DevOps repository for studying and learning Kubernetes manifests. It contains YAML manifests for deploying applications to Kubernetes clusters.

## Project Structure

- `src/`: Kubernetes manifest files (deployments, services, configs)
  - `ror.yaml`: Ruby on Rails application deployment
  - `sample.yml`: EKS sample Linux application with nginx
  - `kredis.yaml`: (currently empty/placeholder)
- `.github/workflows/`: CI pipeline for manifest validation
- `AGENTS.md`: Detailed development guidelines and conventions

## Essential Commands

### Validation and Testing
```bash
# Diff proposed changes against live cluster
kubectl diff -f src/ -R

# Server-side dry run (validates against API)
kubectl apply -f src/ -R --dry-run=server

# Client-side dry run (basic syntax check)
kubectl apply -f src/ -R --dry-run=client

# Validate manifests with kubeconform
kubeconform -strict -summary -ignore-missing-schemas -recursive src/
```

### Deployment Operations
```bash
# Apply a single manifest
kubectl apply -f src/ror.yaml -n default

# Apply all manifests
kubectl apply -f src/ -R

# Inspect deployments
kubectl get deploy -n default
kubectl describe deploy/app-ror
```

## Architecture and Conventions

### Manifest Structure
- All manifests use explicit `metadata.namespace` declarations
- Resource naming follows kebab-case convention
- Images should avoid `:latest` tags; prefer pinned versions or digests
- Resource requests/limits are required for all containers

### File Naming
- Prefer `.yaml` extension over `.yml` (migrating legacy files)
- Use kebab-case for filenames
- Group related resources in single files or clear pairs

### Labels and Selectors
- Standard labels: `app`, `run`, `env`
- Keep label selectors consistent across resources
- Example: `app: eks-sample-linux-app`

## CI/CD Pipeline

The `.github/workflows/blank.yml` workflow runs on push/PR to main branch:
1. Client-side dry-run validation
2. kubeconform schema validation with strict mode
3. Uses kubectl v1.30.0 and kubeconform v0.6.7

## Development Workflow

1. Before making changes: run `kubectl diff` on affected files
2. Make manifest changes following AGENTS.md conventions
3. Test locally with `--dry-run=server` validation
4. Commit using Conventional Commits format
5. CI will validate all manifests automatically

## Security Notes

- Never commit secrets; reference Secret/ConfigMap resources instead
- Document any required prerequisites (CRDs, namespaces) in PRs
- Prefer immutable image digests for production workloads