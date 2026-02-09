# Repository Guidelines

## Purpose
This document is the source of truth for contributor workflow at workspace level. The repository has three crates with different responsibilities:

- `crates/teloxide-ng`
- `crates/teloxide-core-ng`
- `crates/teloxide-macros-ng`

Use this file first, then follow crate-specific `AGENTS.md` in the crate you edit.

## Source of Truth and Synchronization Policy
Keep `AGENTS.md` files synchronized.

- If root-level process changes (CI jobs, commands, workflow, release/check policy), update:
  - `AGENTS.md` (root)
  - crate files that reference those commands/policies
- If crate-level process changes (tests/features/commands), update:
  - the crate `AGENTS.md`
  - root `AGENTS.md` if the change affects shared workflow or onboarding
- Do these updates in the same PR to avoid stale instructions.

## Ask-Before-Change Policy
For ambiguous or high-impact changes, ask first in issue/PR discussion before implementing broad edits.

Ask before:
- breaking API behavior
- changing feature defaults
- touching multiple crates for non-wiring reasons
- changing CI workflow behavior

For small local fixes inside one crate, proceed directly.

## Workspace Layout
- `crates/teloxide-ng`: high-level framework APIs, dispatching, listeners, utils, integration tests, examples
- `crates/teloxide-core-ng`: Telegram Bot API models/payloads/requests/net/adaptors/codegen inputs
- `crates/teloxide-macros-ng`: procedural macros used by `teloxide-ng`
- `.github/workflows/ci.yml`: CI source of truth
- `.cargo/config.toml`: cargo aliases (including `docs`)
- `Justfile`: local shortcuts mirroring CI checks
- `CONTRIBUTING.md`, `CODE_STYLE.md`, `CHANGELOG.md`, `MIGRATION_GUIDE.md`: mandatory contributor references

## CI Requirements (Authoritative)
Workflow: `Continuous integration` on `push`, `pull_request`, and `merge_group` for `master`.

### Jobs
- `fmt`
- `test`
- `check-examples`
- `clippy`
- `doc`
- `ci-pass` aggregator (depends on all jobs above)

### CI Environment
- `RUSTFLAGS="--cfg CI_REDIS --cfg CI_POSTGRES -Dwarnings"`
- `RUSTDOCFLAGS="--cfg docsrs -Dwarnings"`
- `CARGO_INCREMENTAL=0`
- `CARGO_NET_RETRY=10`
- `RUSTUP_MAX_RETRIES=10`
- `CI=1`

### Toolchain Matrix and Features
`test` job runs on:
- `stable` with `--features full`
- `beta` with `--features full`
- `nightly` with `--features "full nightly"`

On `stable` and `beta`, CI skips codegen-heavy tests:
- `local_macros::codegen_requester_forward`
- `requests::requester::codegen_requester_methods`

### External Services in CI
- PostgreSQL service container (`localhost:5432`, user `teloxide`, password `rewrite_it_in_rust`)
- Redis installed in job and started on ports `7777`, `7778`, `7779`
- Databases created in CI:
  - `test_postgres_json`
  - `test_postgres_bincode`
  - `test_postgres_cbor`

## Local Commands (Preferred)
Run from workspace root.

- `just fmt`
- `just lint`
- `just test`
- `just docs`
- `just ci`

Equivalent cargo commands:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --features "full nightly"`
- `cargo test --features "full nightly"`
- `cargo docs` (uses `docs` alias from `.cargo/config.toml`)

## CI-Accurate Local Reproduction
For maximum parity with CI:

1. Install toolchains: `stable`, `beta`, `nightly`
2. Set env flags:
   - `RUSTFLAGS="--cfg CI_REDIS --cfg CI_POSTGRES -Dwarnings"`
   - `RUSTDOCFLAGS="--cfg docsrs -Dwarnings"`
3. Start Redis on `7777`, `7778`, `7779`
4. Start PostgreSQL and create test DBs listed above
5. Run matrix-style commands (`cargo +stable/...`, `cargo +beta/...`, `cargo +nightly/...`)

## Release Readiness
Before publishing new crate versions:

1. Confirm all crate names/paths remain `*-ng` in manifests, docs, and examples.
2. Run: `cargo fmt --all -- --check`, `cargo check --workspace --features "full nightly"`, `cargo clippy --workspace --features "full nightly" --all-targets -- -D warnings`.
3. Run tests using CI-style split:
   - `cargo test --tests --features "full nightly"`
   - `cargo test --doc --features "full nightly"`
4. Run `just check-no-email`.
5. If codegen tests touched files (`requester.rs`, `local_macros.rs`), re-run tests once more and commit generated updates.

## Style and Stability Rules
- Rust 2024 edition, MSRV `1.85`
- Follow `CODE_STYLE.md` and `rustfmt.toml`
- `snake_case` for modules/functions, `PascalCase` for types/traits
- Avoid `unwrap`/`expect` in library logic
- Prefer additive API changes and minimal scope

## Commit and PR Rules
Use Conventional Commit style (`feat:`, `fix:`, `chore:`, etc.).

PRs should include:
- clear summary and rationale
- linked issue when applicable (`Fixes #123`)
- validation commands run locally
- changelog + migration updates for user-visible/breaking changes

Breaking changes must be marked with `[**BC**]` in changelog and reflected in `MIGRATION_GUIDE.md`.

## Security and Configuration
- Never commit secrets (`TELOXIDE_TOKEN`, DB credentials, tokens)
- Use environment variables
- Avoid logging sensitive payloads
- Treat proc-macro diagnostics and unsafe-related edits as high-priority review areas

## No-Email Policy
- Do not commit email addresses to repository files.
- Local enforcement: run `just check-no-email`.
- CI enforcement: workflow job `check-no-email` is required by `ci-pass`.
- If a tool or config requires contact metadata, prefer non-email identifiers or remove that integration.

## Toolchain Profile Clarification
`rust-toolchain.toml` is repository-wide rustup configuration, not a user profile system.

- `channel = "nightly"`: pins toolchain expected by CI/docs/lint flow.
- `profile = "minimal"`: rustup install profile (fewer default components), keeps environments lean and reproducible.


