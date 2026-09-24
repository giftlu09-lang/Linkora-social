# Contributing to Linkora

Thank you for your interest in contributing to Linkora! This guide covers everything you need to get your development environment running, work with branches, submit pull requests, and keep the project's tooling happy.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Branch Naming Conventions](#branch-naming-conventions)
4. [Making Changes](#making-changes)
5. [Pull Request Process](#pull-request-process)
6. [Lockfile Policy](#lockfile-policy)
7. [Environment Protection (Production)](#environment-protection-production)
8. [Code Style](#code-style)
9. [Running Tests](#running-tests)

---

## Prerequisites

Make sure the following tools are installed before you begin:

| Tool                | Minimum version     | Notes                                        |
| ------------------- | ------------------- | -------------------------------------------- |
| Node.js             | See `.node-version` | Use `fnm` or `nvm` to pin the exact version  |
| pnpm                | 9.x                 | `npm install -g pnpm`                        |
| Rust + Cargo        | stable              | `rustup toolchain install stable`            |
| Docker + Compose v2 | any recent          | Required for migration tests only            |
| stellar-cli         | latest              | Required for integration tests only          |
| GitHub CLI (`gh`)   | 2.x                 | Required to create PRs from the command line |

---

## Environment Setup

```bash
# 1. Clone the repository
git clone https://github.com/giftlu09-lang/Linkora-social.git
cd Linkora-social

# 2. Run the automated setup script (checks prerequisites, installs deps, builds contracts)
./scripts/setup.sh

# 3. Verify everything is working
pnpm typecheck
pnpm test
pnpm build
```

### Manual setup (if setup.sh is not suitable for your environment)

```bash
# Install all JS/TS dependencies
pnpm install

# Build Soroban contracts
cd packages/contracts
cargo build --target wasm32v1-none --release -p linkora-contracts
cd ../..

# Build all JS/TS packages
pnpm build
```

### Service-specific setup

```bash
# Web frontend (http://localhost:3000)
cd apps/web && pnpm dev

# Mobile app (Expo)
cd apps/mobile && pnpm start   # press 'a' for Android, 'i' for iOS

# Indexer
cd services/indexer
cp .env.example .env            # fill in DATABASE_URL and SOROBAN_RPC_URL
pnpm dev
```

---

## Branch Naming Conventions

Use one of the following prefixes so that PRs are easy to categorise and CI can apply the right checks:

| Prefix      | When to use                                 | Example                            |
| ----------- | ------------------------------------------- | ---------------------------------- |
| `feat/`     | New features or capabilities                | `feat/280-pnpm-lockfile-ci`        |
| `fix/`      | Bug fixes                                   | `fix/312-follow-count-overflow`    |
| `chore/`    | Tooling, dependency bumps, config changes   | `chore/bump-pnpm-9.5`              |
| `docs/`     | Documentation-only changes                  | `docs/update-architecture-diagram` |
| `refactor/` | Code restructuring without behaviour change | `refactor/sdk-transaction-queue`   |
| `test/`     | Adding or updating tests                    | `test/add-tip-fuzz-cases`          |

Include the issue number in the branch name when one exists (e.g. `feat/280-pnpm-lockfile-ci`).

---

## Making Changes

1. **Create a branch** from an up-to-date `main`:

   ```bash
   git checkout main
   git pull origin main
   git checkout -b feat/<issue-number>-short-description
   ```

2. **Make your changes.** Keep commits focused — one logical change per commit.

3. **Follow conventional commits** for commit messages:

   ```
   <type>(<scope>): <short summary> (#<issue>)

   # Examples
   feat(ci): add pnpm lockfile verification step (#280)
   fix(contracts): clamp follow-count to u32::MAX (#312)
   docs(contributing): add lockfile update guidance (#280)
   ```

   Valid types: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `perf`, `ci`.

4. **Run checks locally** before pushing:

   ```bash
   pnpm typecheck
   pnpm lint
   pnpm test
   pnpm build
   ```

5. **Push your branch:**

   ```bash
   git push -u origin feat/<issue-number>-short-description
   ```

---

## Pull Request Process

1. Open a PR against `main` using the GitHub UI or the CLI:

   ```bash
   gh pr create --title "feat(scope): short summary" \
     --body "..." \
     --base main \
     --head feat/<branch-name>
   ```

2. Fill in the PR description template with:
   - **Summary** — what the change does and why.
   - **Changes** — a bullet list of the files/areas affected.
   - **Testing** — how you tested the change.
   - **Closes #\<issue\>** — links the PR to the relevant issue.

3. All CI checks must pass before a PR can be merged:
   - `lockfile-check` — pnpm lockfile in sync with all `package.json` files.
   - `js-ci` — TypeScript typechecking, tests, and build.
   - `lint` — ESLint / Prettier.
   - `unit-tests` — Rust contract unit and fuzz tests.

4. At least one maintainer approval is required.

5. Squash-merge is preferred for feature branches to keep `main` history clean. Maintainers will do this on merge.

---

## Lockfile Policy

### Why the lockfile matters

`pnpm-lock.yaml` records the exact resolved version of every dependency in the workspace. A lockfile that is out of sync with `package.json` files means different developers — and CI — may install different package versions, leading to subtle, hard-to-debug inconsistencies.

### CI enforcement

The `lockfile-check` CI job runs `pnpm install --frozen-lockfile` on every push and pull request. `--frozen-lockfile` instructs pnpm to fail immediately (exit code 1) if `pnpm-lock.yaml` does not match the current `package.json` files. **This job will always fail if you forget to commit an updated lockfile.**

### How to update the lockfile

Whenever you add, remove, or change a dependency in any `package.json`:

```bash
# 1. Make your changes to the relevant package.json
#    e.g. pnpm add some-library --filter apps/web

# 2. Let pnpm regenerate the lockfile
pnpm install

# 3. Stage BOTH the package.json and the lockfile
git add <path-to-package.json> pnpm-lock.yaml

# 4. Commit them together
git commit -m "chore(deps): add some-library to apps/web"
```

**Never commit a `package.json` change without also committing the updated `pnpm-lock.yaml`.** If you see a CI failure on the `lockfile-check` job, this is the cause — run `pnpm install` locally and push the updated lockfile.

### Do not manually edit pnpm-lock.yaml

`pnpm-lock.yaml` is machine-generated. Editing it by hand will almost certainly corrupt it. Always let `pnpm install` regenerate it.

---

## Environment Protection (Production)

Deployments to the **production** environment require an explicit approval from a maintainer before they can proceed. This is enforced via GitHub's [environment protection rules](https://docs.github.com/en/actions/deployment/targeting-different-environments/using-environments-for-deployment#required-reviewers).

- **staging** — deploys automatically on merge to `main`.
- **production** — requires at least one maintainer approval in the GitHub UI after a staging deployment succeeds.

If you are a maintainer and need to approve a production deployment, navigate to the Actions run, find the pending deployment gate, and click **Review deployments → Approve**.

Do not attempt to bypass environment gates by modifying workflow files. Such PRs will be rejected.

---

## Code Style

- **TypeScript**: ESLint + Prettier. Run `pnpm lint` to check, `pnpm lint --fix` to auto-fix.
- **Rust**: `rustfmt` + Clippy. The CI runs `cargo fmt --check` and `cargo clippy -- -D warnings`. Run `cargo fmt` locally before pushing.
- **Commit messages**: Follow the conventional commits format described above.
- **File headers**: No license headers required in individual files — the project-level `LICENSE` (MIT) covers all source files.

---

## Running Tests

```bash
# TypeScript unit tests (all packages)
pnpm test

# Rust contract unit tests + fuzz / invariant tests
pnpm --filter contracts test
# or
cd packages/contracts && cargo test

# Indexer database migration tests (requires Docker)
bash tests/migrations/test-migrations.sh
```

See [`services/indexer/migrations/README.md`](./services/indexer/migrations/README.md) for details on the migration testing setup.

---

## Questions?

Open a discussion on GitHub or join the [Telegram community](https://t.me/+13csp8G4ccRhY2Zk).
