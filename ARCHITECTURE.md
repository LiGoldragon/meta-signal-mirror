# meta-signal-mirror — Architecture

`meta-signal-mirror` is the meta policy wire contract of the mirror triad
(`mirror` runtime, `signal-mirror` ordinary contract, `meta-signal-mirror`
meta policy contract). Schema-derived: `schema/lib.schema` is the source,
`build.rs` drives `schema_rust_next::build::ContractCrateBuild`
(`WireContract` target), generated module checked in at `src/schema/lib.rs`.
It cites `primary/skills/component-triad.md` and
`primary/skills/contract-repo.md`.

## The relation

One relation: **owner ↔ mirror daemon**, over the owner-only Unix meta
socket (mode `0o600`). The meta surface is structurally unreachable over the
tailnet TCP ingress — the TCP listener decodes only the ordinary
`signal-mirror` contract.

- **Endpoints.** The owner (deploy tooling, the meta CLI) sends; the mirror
  daemon replies.
- **Authority.** Owner-only: store registration/retirement, retention
  policy, configuration. Kernel-vouched Unix peer credentials plus the
  socket mode are the boundary; no payload claim is trusted.
- **Lifecycle vectors.** Configured, StoreRegistered, StoreRetired,
  RetentionSet, RegistryObserved, OrderRejected.

## DaemonConfiguration

The contract declares the daemon's typed configuration record: storage
path, working Unix socket path + mode, meta Unix socket path + mode, and
the tailnet TCP binding address. The daemon's single startup argument is a
binary rkyv archive of this record (one-argument rule; the daemon never
parses NOTA). `ConfigurationWrite` is the deploy text-edge request the
`mirror-write-configuration` helper consumes to produce that binary file.
The same record rides the meta `Configure` operation.

## Retention — typed placeholder

`RetentionOrder` (scope + rule) is named, wire-typed, and stored by the
daemon, but NOT enforced in this cut — enforcement is deferred by decision
(see `INTENT.md`). The vocabulary exists so policy can be installed and
observed before the pruning machinery lands.

## Code map

| Path | What |
|---|---|
| `schema/lib.schema` | the authored contract source |
| `build.rs` | `ContractCrateBuild` — regenerate with `META_SIGNAL_MIRROR_UPDATE_SCHEMA_ARTIFACTS=1 cargo build` |
| `src/schema/lib.rs` | generated wire types + signal-frame codec (never hand-edited) |
| `src/lib.rs` | re-exports + binary configuration archive helpers |
| `tests/round_trip.rs` | rkyv frame + NOTA text round-trips per operation |

## Not owned

No runtime, no actors, no tokio, no enforcement logic. The daemon owns the
registry's durable state and the (deferred) retention enforcement.
