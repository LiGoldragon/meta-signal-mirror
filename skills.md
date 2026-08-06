# Skill — meta-signal-mirror

## Ownership

This repository owns the owner Mirror Interface only: configuration, store
registration and retirement, registry observation, and retention vocabulary.
Runtime state and enforcement belong to `mirror`; ordinary working traffic
belongs to `signal-mirror`.

## Editing

- Edit `ethos/interface.ethos`; no second schema language or emitter exists.
- Preserve the role-free source. Request/reply roles are current-stage Signal
  behavior.
- Reuse producer identities through exact imports. Never copy `StoreName`,
  `SocketPath`, or `NetworkEndpoint` locally.
- Mint declaration seats explicitly. Never derive identity or canonical order
  from a name, position, or source contents.
- Regenerate with
  `META_SIGNAL_MIRROR_UPDATE_INTERFACE_ARTIFACTS=1 cargo build` only when the
  authority transaction intentionally changes.
- Keep Dotos as the sole optional text projection.

## Proof

Run `cargo test --all-features --all-targets`, both default and all-feature
Clippy with warnings denied, rustdoc with warnings denied, and
`nix flake check --all-systems`. A final build without the update variable must
prove the checked projection is fresh.
