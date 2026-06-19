# Skill — meta-signal-mirror

*How to work in the mirror triad's owner-only meta policy contract.*

## What This Repo Owns

`meta-signal-mirror` owns the schema-derived meta policy wire vocabulary for
the `mirror` daemon. It is the owner-only Unix-socket contract for daemon
configuration, store registration and retirement, registry observation, and
the typed retention placeholder.

The repo owns wire types and codecs only. Runtime behavior, daemon state,
actors, socket serving, registration persistence, and retention enforcement
belong to `mirror`.

## Editing Rules

Edit `schema/lib.schema` for contract shape changes and regenerate the Rust
artifact with:

```sh
META_SIGNAL_MIRROR_UPDATE_SCHEMA_ARTIFACTS=1 cargo build
```

Do not hand-edit `src/schema/lib.rs`; it is generated. Handwritten behavior
belongs in `src/lib.rs` only when it is a method or trait impl on a generated
or real data-bearing type.

Use remote git dependencies only. Cross-repo Cargo dependencies stay as
`git = "https://github.com/..."` entries with `Cargo.lock` carrying the exact
resolved revision; do not add local `path = "../..."` overrides.

There is no runtime crate in this repo: no `tokio`, no actors, no daemon
startup code, and no policy-enforcement logic.

## Retention Placeholder

`RetentionOrder` is typed policy vocabulary that the daemon can store and
report. It is not enforcement. Do not describe retention as active pruning
until the runtime implements the pruning machinery in `mirror`.

The meta surface is Unix-owner-only. Do not add TCP-reachable meta operations
or ordinary working-signal verbs here.

## Verification

Run the local contract checks before committing:

```sh
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

This repo currently has no `flake.nix`; `nix flake check` is not available
inside the repo unless a future change adds a flake.

## See Also

- `INTENT.md` — psyche-backed role and constraints for this contract.
- `ARCHITECTURE.md` — relation, code map, and boundary description.
- `AGENTS.md` — required repo-specific agent notes.
