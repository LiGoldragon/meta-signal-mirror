# Skill — meta-signal-mirror

## Ownership

This repository owns the owner Mirror Interface only: configuration, store
registration and retirement, registry observation, and retention vocabulary.
Runtime state and enforcement belong to `mirror`; ordinary working traffic
belongs to `signal-mirror`.

## Editing

- Edit `ethos/signal.ethos`; no second schema language or emitter exists.
- The source is a `Signal` root; request and reply seating is in the source
  itself, not in behavior.
- Reuse producer identities through exact imports. Never copy `StoreName`,
  `SocketPath`, or `NetworkEndpoint` locally.
- Regenerate `src/generated/signal.rs` with `ethos-zero`; `build.rs` asserts the
  checked-in projection against a fresh generation.
- Update `examples/canonical.datom` whenever a root changes; `tests/canonical.rs`
  will not compile until every root has a line.
- Datom is the only text projection. Take the portable frame from `signal`;
  never restate it here.

## Proof

Run `cargo test --all-features --all-targets`, both default and all-feature
Clippy with warnings denied, rustdoc with warnings denied, and
`nix flake check`. The build itself proves the checked projection is fresh.
