# Repository Guidelines

## Crate Purpose
`teloxide-macros` provides procedural macros used by `teloxide`, primarily command-related derives and attribute handling.

Main quality target: predictable macro behavior and clear compile-time diagnostics.

## Crate Structure
- `src/lib.rs`: proc-macro entry points
- `src/bot_commands.rs`, `src/command.rs`, `src/command_enum.rs`, `src/command_attr.rs`: derive parsing and expansion
- `src/attr.rs`, `src/fields_parse.rs`: attribute and field parsers
- `src/rename_rules.rs`: rename logic + tests
- `src/error.rs`: diagnostic helpers
- `src/unzip.rs`: helper utilities used across parser paths

Keep parsing, validation, and generation paths separated.

## Scope and Cross-Crate Rule
Most macro implementation changes should stay in this crate.

Touch `teloxide` only when:
- macro integration tests or expected behavior docs/examples need updates
- new macro behavior requires consumer-side adjustments

Avoid broad cross-crate edits unless strictly required.

## Ask-Before-Change Rule
Ask in issue/PR discussion before:
- changing accepted macro syntax
- changing default command parsing semantics
- altering attribute conflict behavior
- making changes that can break existing derive usage patterns

Proceed directly for local bug fixes and diagnostics improvements.

## CI Expectations for This Crate
Direct and indirect CI coverage:

1. workspace `test` matrix (`stable`, `beta`, `nightly`)
2. workspace `clippy` (`--all-targets --features "full nightly"`)
3. workspace `fmt` and `doc`
4. integration confidence via `crates/teloxide/tests/command.rs`

Even when this crate compiles alone, regressions may appear only in consumer integration tests.

## Local Commands (Recommended)
Run from workspace root.

- `cargo fmt --all`
- `cargo clippy -p teloxide-macros --all-targets`
- `cargo test -p teloxide-macros`
- `cargo test -p teloxide --test command --features macros`
- `just ci`

For maximum parity, run tests with nightly and ensure docs/clippy at workspace level remain green.

## Macro Implementation Rules
- Prefer explicit `syn` parsing paths over hidden fallback behavior
- Preserve spans so diagnostics point to user code
- Avoid panic-based failures in macro internals
- Emit actionable compile errors
- Keep generated token streams small and reviewable
- Document default behavior and attribute conflicts when adding new options

## Naming and Style
- Follow root `CODE_STYLE.md` where applicable
- `snake_case` helper names, `PascalCase` data structures
- use explicit helper naming: `parse_*`, `validate_*`, `expand_*`
- keep large expansion logic split into focused helpers

## Testing Guidance
- Add unit tests near changed parser modules
- Validate end-user behavior in `crates/teloxide/tests/command.rs`
- Cover success and failure cases (invalid attrs, separator conflicts, etc.)
- If diagnostics changed, verify message clarity and span quality

## Backward Compatibility
Macro syntax/semantics is user-facing API. Treat these as breaking:
- accepting/rejecting different attribute forms
- changing defaults for rename/prefix/parse behavior
- changing parse outcomes for previously valid command definitions

If intentional, document migration impact and mark changelog accordingly.

## Commit and PR Checklist
- Conventional Commit subject
- before/after usage examples
- compatibility notes
- tests proving behavior
- mention integration test result from consumer side
- `[**BC**]` + migration notes when breaking

## Sync Rule for AGENTS Files
If macro-related CI expectations or contribution process changes, update:
- `crates/teloxide-macros/AGENTS.md`
- root `AGENTS.md`
in the same PR.
