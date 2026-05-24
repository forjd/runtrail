# Agent Instructions

## Project overview

`runtrail` is a small Rust CLI for writing, inspecting, summarising, diffing, and replaying JSONL event trails for agentic dev, browser QA, repo, and CI workflows.

- Crate: `runtrail`
- Binary: `runtrail` (`src/main.rs`)
- Rust edition: 2024
- Default event log path: `.runtrail/events.jsonl`

## Repository layout

- `src/cli.rs` - Clap command definitions and command dispatch.
- `src/event.rs` - event envelope types, schema defaults, and event construction.
- `src/log_io.rs` - JSONL read/write/validation logic.
- `src/summary.rs`, `src/diff.rs`, `src/replay.rs`, `src/repair.rs`, `src/index.rs` - command feature modules.
- `src/command_run.rs`, `src/git.rs`, `src/ci_capture.rs`, `src/redaction.rs` - evidence capture helpers.
- `tests/cli.rs` - integration tests for the CLI.
- `docs/` - schema, conventions, GitHub Actions guide, plans, and research notes.
- `examples/` - sample JSONL trails.
- `scripts/perf-smoke.sh` - release-mode performance smoke test.

## Development commands

Run these from the repository root:

```bash
cargo fmt
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
scripts/perf-smoke.sh 10000
```

CI enforces the check variants:

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
scripts/perf-smoke.sh 10000
```

Useful local commands:

```bash
cargo build --release --locked
cargo run -- --help
cargo run -- log --event agent.note --message "hello"
cargo run -- validate --file .runtrail/events.jsonl
```

## Coding guidelines

- Git commit messages must always follow the Conventional Commits standard.
- Keep the CLI small, deterministic, and shell-friendly.
- Preserve backwards compatibility for documented flags and output formats when possible.
- Prefer clear module-level responsibilities over putting new behavior directly in `main.rs`.
- Add or update integration tests in `tests/cli.rs` for CLI-visible behavior.
- Use `tempfile` in tests rather than writing persistent logs in the repository.
- Keep generated or local evidence under `.runtrail/`; do not commit it unless explicitly requested.
- Avoid capturing secrets. CI capture should remain allowlist-based.
- When adding event fields or event types, update `docs/schema-v1.md` or `docs/event-conventions.md` as appropriate.

## Event format expectations

Each event is one JSON object per line. Preserve these conventions:

- `schema` should remain `runtrail.v1` unless doing a deliberate schema migration.
- `seq` is a positive sequence number within the log file.
- `ts` is RFC3339 UTC.
- `event` names are dotted names such as `agent.note` or `command.end`.
- `level` is one of `trace`, `debug`, `info`, `warn`, or `error`.
- `attrs` and `body` are always JSON objects; use `{}` when empty.
- Prefer compact, stable JSON output that is easy to diff and pipe through tools.

## Documentation expectations

When changing behavior that users see, check whether these need updates:

- `README.md` for command examples and quick-start flows.
- `docs/schema-v1.md` for envelope or payload shape changes.
- `docs/event-conventions.md` for naming or semantic conventions.
- `docs/github-actions.md` for CI workflow behavior.
- `CHANGELOG.md` only when the project convention calls for a manual entry; releases are also configured through release-please.

## Before handing off

- Run formatting and the most relevant tests for your change.
- If the change touches CLI behavior, run at least `cargo test --locked`.
- If the change could affect performance of validation or summarisation, run `scripts/perf-smoke.sh 10000`.
- Summarize changed files and any commands you ran.
