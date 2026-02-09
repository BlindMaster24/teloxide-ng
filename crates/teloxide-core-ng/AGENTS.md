# Repository Guidelines

## Crate Purpose
`teloxide-core-ng` is the low-level Telegram Bot API client crate. It owns protocol models, method payloads, request abstractions, network layer, and adaptors.

Protocol-level accuracy and backward compatibility are the top priorities for this crate.

## Crate Structure
- `src/lib.rs`: public exports and crate docs
- `src/types/`: Telegram API object models
- `src/payloads/`: request payloads for Bot API methods
- `src/requests/`: requester traits and request abstractions
- `src/net/`: HTTP request/response handling
- `src/adaptors/`: throttle, trace, erased, cache adaptors
- `src/codegen/` + `schema.ron`: codegen-related inputs and transforms
- `examples/`: focused usage examples

## Scope and Cross-Crate Rule
Change this crate first for Bot API protocol updates. Touch `teloxide-ng` only when high-level exposure/wiring is required.

Avoid unrelated refactors during API update work.

## Ask-Before-Change Rule
Ask in issue/PR discussion before:
- removing/renaming public items
- changing defaults affecting downstream crates
- broad reshapes of requester/adaptor surfaces
- altering codegen workflow behavior

Proceed directly for local bug fixes and additive changes.

## CI Expectations for This Crate
This crate is heavily exercised by the CI `test` matrix.

### Toolchains and Features
- `stable`: `--features full`
- `beta`: `--features full`
- `nightly`: `--features "full nightly"`

### Codegen Test Behavior
On `stable` and `beta`, CI skips:
- `local_macros::codegen_requester_forward`
- `requests::requester::codegen_requester_methods`

On `nightly`, these are not skipped.

### Additional CI Context
- `RUSTFLAGS` includes `-Dwarnings` and cfgs for CI service tests
- docs job uses `RUSTDOCFLAGS="--cfg docsrs -Dwarnings"`
- examples and no-default-features checks run at workspace level and can fail due to this crate’s changes

## Local Commands (Recommended)
Run from workspace root.

- `cargo fmt --all`
- `cargo clippy -p teloxide-core-ng --all-targets --features "full nightly"`
- `cargo test -p teloxide-core-ng --features "full nightly"`
- `cargo test -p teloxide-core-ng --doc --features "full nightly"`
- `cargo check -p teloxide-core-ng --examples --features "full nightly"`
- `just ci`

For CI parity, also run toolchain-specific commands with `+stable`, `+beta`, `+nightly`.

## Bot API Update Workflow (Practical)
1. Read `CONTRIBUTING.md` TBA section.
2. Add/update `types` first.
3. Add/update `payloads` and request wiring.
4. Run tests to validate codegen-dependent parts.
5. Fix forwarding/requester compile gaps.
6. Add regression tests.

## Modeling, Serde, and Stability Rules
- Keep names aligned with Telegram API in `snake_case` filenames
- Use explicit serde attributes where protocol requires
- Keep derives consistent with surrounding models
- Do not force `Eq`/`Hash` where field semantics do not allow it
- Prefer additive changes to preserve downstream compatibility

## Testing Guidance
High-value tests:
- serde round-trip for changed types
- payload serialization shape checks
- response parsing behavior in `net`
- adaptor/request forwarding correctness

If touching requester/codegen layers, run full test suite to catch missing forwarding points.

## Commit and PR Checklist
- Conventional Commit subject with scope clarity
- summary of protocol/API change
- compatibility notes for downstream crates
- local commands executed
- changelog/migration updates for user-visible or breaking changes (`[**BC**]` when needed)

## Sync Rule for AGENTS Files
If CI commands, workflow assumptions, or protocol-update process changes here, update:
- `crates/teloxide-core-ng/AGENTS.md`
- root `AGENTS.md`
in the same PR.


