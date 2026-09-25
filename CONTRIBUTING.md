# Contributing to Linkora

Thank you for contributing! This guide covers everything you need to set up your environment, follow project conventions, and get your changes merged.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Branch Conventions](#branch-conventions)
- [Commit Style](#commit-style)
- [Running Tests](#running-tests)
- [Lockfile Management](#lockfile-management)
- [Production Deployments](#production-deployments)
- [Pull Request Process](#pull-request-process)
- [Code Style](#code-style)

---

## Prerequisites

| Tool | Minimum version | Notes |
|------|----------------|-------|
| Node.js | See `.node-version` | Use `nvm use` or `fnm use` to match exactly |
| pnpm | 9.x | `npm install -g pnpm` |
| Rust + Cargo | stable | `rustup update stable` |
| `wasm32v1-none` target | — | `rustup target add wasm32v1-none` |
| stellar-cli | latest | `cargo install --locked stellar-cli` |
| Docker + Compose v2 | — | Required for migration tests only |
| GitHub CLI (`gh`) | 2.x | Required to create PRs from CLI |

---

## Quick Start

```bash
# Clone and run the automated setup (checks prerequisites, installs deps, builds contracts)
git clone https://github.com/giftlu09-lang/Linkora-social.git
cd Linkora-social
./scripts/setup.sh

# Start the web frontend
cd apps/web && pnpm dev           # http://localhost:3000

# Start the mobile app
cd apps/mobile && pnpm start      # press 'a' for Android, 'i' for iOS

# Start the indexer
cd services/indexer
cp .env.example .env              # fill in DATABASE_URL and SOROBAN_RPC_URL
pnpm dev
```

---

## Branch Conventions

Branch names must follow this pattern: `<type>/<issue-number>-<short-description>`

| Type | When to use |
|------|-------------|
| `feat/` | New features |
| `fix/` | Bug fixes |
| `chore/` | Maintenance, dependency bumps, tooling |
| `docs/` | Documentation-only changes |
| `refactor/` | Code restructuring without behaviour change |
| `test/` | Adding or fixing tests |

Examples:
- `feat/282-env-protection`
- `fix/301-profile-fetch-race`
- `docs/310-indexer-query-examples`

Always branch from `main`:

```bash
git checkout main && git pull
git checkout -b feat/<issue>-<description>
```

---

## Commit Style

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary> (#<issue>)
```

Examples:
```
feat(ci): add GitHub Actions environment protection for production (#282)
fix(contracts): correct fee calculation overflow (#301)
docs(indexer): add query examples to README (#310)
```

Scopes map to monorepo packages/apps/services:
`contracts`, `sdk`, `web`, `mobile`, `indexer`, `dm-relay`, `analytics-oracle`, `ci`, `docs`

---

## Running Tests

### JavaScript / TypeScript

```bash
# All JS/TS tests across the monorepo
pnpm test

# Single package
pnpm --filter sdk test
pnpm --filter web test
```

### Rust contracts

```bash
pnpm --filter contracts test
# or equivalently:
cd packages/contracts && cargo test
```

### Database migration tests

Spins up a throwaway PostgreSQL container, applies all migrations forward, checks the schema against the committed snapshot, verifies idempotency on re-apply, and tears everything down.

```bash
bash tests/migrations/test-migrations.sh
```

Requires Docker and Compose v2. See [`services/indexer/migrations/README.md`](./services/indexer/migrations/README.md) for authoring rules and how to refresh the schema snapshot.

### End-to-end tests

```bash
pnpm --filter web test:e2e
pnpm --filter sdk test:e2e
```

---

## Lockfile Management

### Why the lockfile is committed

`pnpm-lock.yaml` is committed to the repository. It guarantees reproducible installs across all environments — CI, dev machines, and production — so that everyone runs exactly the same dependency tree.

### How CI enforces it

The CI pipeline has a dedicated `lockfile-check` job that runs on every PR. It:

1. Runs `pnpm install --frozen-lockfile` — aborts with a clear error if `pnpm-lock.yaml` does not match any `package.json` in the workspace.
2. Runs `git diff --exit-code pnpm-lock.yaml` — fails if the install generated any changes to the lockfile.

Both the `js-ci` and `lint` jobs declare `needs: lockfile-check`, so they are skipped entirely when the lockfile check fails, giving a fast and unambiguous failure signal.

### Updating the lockfile

After adding, removing, or changing any dependency in a `package.json`:

```bash
# Re-generate the lockfile
pnpm install

# Verify it's consistent before committing
pnpm install --frozen-lockfile

# Stage and commit the updated lockfile together with package.json
git add pnpm-lock.yaml packages/<name>/package.json
git commit -m "chore(<scope>): add/update <package> dependency"
```

**Never** run `pnpm install` without also committing the resulting `pnpm-lock.yaml` changes.

### Fixing a failing `lockfile-check` job

1. Pull the latest changes from the remote branch.
2. Run `pnpm install` locally to regenerate the lockfile.
3. Commit `pnpm-lock.yaml` alongside the `package.json` that triggered the drift.
4. Push — the check will pass on the next run.

---

## Production Deployments

### Overview

Production deployments are gated behind a GitHub Actions `production` environment with required-reviewer approval. No one can deploy to production unilaterally.

The workflow is at `.github/workflows/deploy-production.yml` and is triggered manually only (`workflow_dispatch`).

### One-time environment setup (repo maintainers)

After the workflow file is merged to `main`, a maintainer must configure the `production` environment in GitHub:

1. Go to **Settings → Environments** in the repository.
2. Click **production** (created automatically on first workflow run, or create it manually).
3. Under **Deployment protection rules**, enable:
   - **Required reviewers** — add at least one maintainer as a required reviewer.
   - **Wait timer** — optional delay before the deployment proceeds (e.g. 5 minutes).
4. Under **Deployment branches**, choose **Selected branches** and add the pattern `main`.
5. Add environment-scoped secrets (**not** repo-level secrets):
   - `ADMIN_SECRET` — admin signing key for contract operations.
   - `TREASURY_ADDRESS` — treasury wallet address.

Full step-by-step instructions are in [`.github/environments/production.md`](.github/environments/production.md).

### Triggering a production deployment

Only maintainers with write access can trigger the workflow:

1. Go to **Actions → Deploy Production** in the repository.
2. Click **Run workflow**.
3. In the confirmation field, type `DEPLOY` (exactly).
4. Click **Run workflow**.
5. The `validate` job runs immediately; the `deploy` job waits for reviewer approval.
6. A required reviewer will receive a notification and must approve (or reject) in the **Environments** tab before the deployment proceeds.

### How the approval gate works

```
workflow_dispatch ──► validate job ──► pending approval ──► deploy job
                                              │
                                    reviewer approves/rejects
                                    (recorded in Environments tab)
```

- While pending, the workflow is paused and shows a yellow "Waiting" badge in Actions.
- The reviewer can approve or reject with a comment.
- Rejection cancels the workflow immediately.
- Only one production deployment can run at a time (`concurrency: group: production`).

### Viewing deployment history

All production deployments are logged automatically. To view history:

- **GitHub UI**: Repository → **Deployments** (left sidebar under Code) → select `production`.
- **Direct link**: `https://github.com/giftlu09-lang/Linkora-social/deployments/activity_log?environment=production`

Each entry shows the deployer, commit SHA, timestamp, and deployment status.

---

## Pull Request Process

1. **All CI checks must pass** — `lockfile-check`, `js-ci`, `lint`, `contracts`, and any other jobs relevant to your change.
2. **At least one approval** from a maintainer is required before merge.
3. **No direct pushes to `main`** — all changes must go through a PR.
4. **Keep PRs focused** — one issue per PR makes reviews faster and rollbacks easier.
5. **Link the issue** — include `Closes #<issue>` in the PR description so the issue closes automatically on merge.
6. **Squash merge** is the default merge strategy on `main`.

### PR description template

```markdown
## Summary

<What this PR does and why>

## Changes

- `path/to/file.ts` — description
- `path/to/other.yml` — description

## Testing

<How you tested this — commands run, what you checked>

Closes #<issue>
```

---

## Code Style

- **TypeScript**: ESLint + Prettier. Run `pnpm lint` to check and `pnpm lint --fix` to auto-fix.
- **Rust**: `rustfmt` + `clippy`. Run `cargo fmt` and `cargo clippy -- -D warnings`.
- **YAML**: 2-space indentation, no trailing whitespace.
- **Markdown**: Wrap at 120 characters where practical.

CI enforces linting on every PR. Fix lint errors locally before pushing to avoid noise.
