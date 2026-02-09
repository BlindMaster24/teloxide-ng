# Repository Guidelines

## Crate Purpose
`teloxide` is the high-level framework crate used by bot developers. It should remain ergonomic, stable, and well-documented for end users.

## Crate Structure
- `src/lib.rs`: public exports and crate docs
- `src/dispatching/`: dispatcher, handlers, dialogues, storage abstractions
- `src/update_listeners/`: polling/webhook listeners
- `src/repls/`: REPL helpers
- `src/sugar/`: ergonomic request helpers
- `src/utils/`: command parsing and markdown/html rendering helpers
- `examples/`: runnable reference bots
- `tests/`: integration tests (`command.rs`, `sqlite.rs`, `redis.rs`, `postgres.rs`)

## Scope and Cross-Crate Rule
Keep changes crate-scoped when possible.

Cross-crate edits are expected only when:
- API wiring requires changes in `teloxide-core` or `teloxide-macros`
- macro behavior changes require integration updates

Do not perform multi-crate refactors unless explicitly justified.

## Ask-Before-Change Rule
Ask in issue/PR discussion before implementing:
- breaking behavior for end users
- feature default changes
- broad dispatching API reshapes
- multi-crate edits not strictly required

Proceed directly for small localized bug fixes.

## CI Expectations for This Crate
This crate is validated by multiple CI jobs:

1. `test` matrix
- `stable` and `beta`: `cargo +<toolchain> test --tests --verbose --features full` (with two codegen test skips)
- `nightly`: `cargo +nightly test --tests --verbose --features "full nightly"`
- doc tests are run separately in same matrix

2. `check-examples`
- `cargo +stable check --examples --features full`
- `cargo +stable check --no-default-features`

3. `clippy`
- workspace-level clippy with `--features "full nightly"`

4. `fmt` and `doc`
- formatting and docs must pass with CI flags (`-Dwarnings`)

CI sets `cfg(CI_REDIS)` and `cfg(CI_POSTGRES)` and uses Redis/Postgres services.

## Local Commands (Recommended)
Run from workspace root.

- `cargo fmt --all`
- `cargo clippy -p teloxide --all-targets --features "full nightly"`
- `cargo test -p teloxide --features "full nightly"`
- `cargo check -p teloxide --examples --features full`
- `cargo test -p teloxide --test command --features macros`
- `just ci`

Storage-focused tests:
- Redis:
  - `cargo test -p teloxide --test redis --features "redis-storage cbor-serializer bincode-serializer"`
- SQLite:
  - `cargo test -p teloxide --test sqlite --features "sqlite-storage-nativetls cbor-serializer bincode-serializer"`
- Postgres:
  - `cargo test -p teloxide --test postgres --features "postgres-storage-nativetls cbor-serializer bincode-serializer"`

## Coding and API Guidelines
- Follow root `CODE_STYLE.md` and `rustfmt.toml`
- Keep handler/filter APIs readable; avoid unnecessary generic complexity
- Avoid panics in library paths
- Keep feature-gated behavior explicit in docs/examples
- For new public APIs, include rustdoc and practical examples when possible

## Testing Guidance
- Unit tests near implementation
- Integration tests for cross-module behavior
- Deterministic assertions preferred
- If command/dialogue behavior changes, update `examples/` or integration tests

## Documentation and Changelog
For user-visible behavior changes:
- update docs/examples in this crate
- add changelog entries at root when applicable
- add migration notes for breaking behavior

## Commit and PR Checklist
- Conventional Commit subject (`feat:`, `fix:`, `chore:`)
- explain behavior change and rationale
- list feature flags touched
- include local validation commands
- mark breaking changes with `[**BC**]` and update `MIGRATION_GUIDE.md`

## Sync Rule for AGENTS Files
If commands, CI assumptions, or crate responsibilities change here, update both:
- `crates/teloxide/AGENTS.md`
- root `AGENTS.md`
in the same PR.
