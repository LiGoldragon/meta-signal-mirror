# meta-signal-mirror — Architecture

`meta-signal-mirror` owns one thing: the vocabulary by which an owner
configures a Mirror and governs its stores. It contains no daemon, actors,
persistence, pruning machinery, or transport server.

## Semantic center

This Interface makes owner authority legible without copying the ordinary
Mirror vocabulary. `StoreName` comes from `signal-mirror`; `SocketPath` and
`NetworkEndpoint` come from `signal-standard`. The relations therefore share
identities, not spellings that merely happen to agree.

`MetaMirrorRequest` and `MetaMirrorReply` remain role-free. Signal request and
reply seating is current-stage Rust behavior, not schema truth. The same Ethos
source must remain meaningful to compilers, agents, harnesses, and visual
interfaces without assuming Rust, LLVM, Signal framing, or this operating
system as its permanent reader.

## The relation

One relation is represented: owner authority over a Mirror daemon.

- Requests configure the daemon, register or retire a store, set retention
  policy, or observe the registry.
- Replies acknowledge the transition, report the registry, or reject an
  order with a typed reason.
- The current daemon exposes this relation through an owner-only local
  endpoint. That access boundary belongs to deployment and runtime policy;
  it is not encoded as an eternal property of the Interface.

The ordinary append, object-notice, checkpoint, restore, and head relations
belong exclusively to `signal-mirror`.

## Configuration shape

`DaemonConfiguration` composes four semantic values:

- `StoragePath`
- `WorkingSocketBinding`
- `MetaSocketBinding`
- the shared `NetworkEndpoint`

Both local bindings wrap the same `LocalSocketBinding` of shared `SocketPath`
and local `SocketMode`. The distinct wrappers preserve why each binding exists
while sharing its structure. The binary archive helpers are an adapter for the
current daemon startup path; they are behavior over the Interface, not the
authority from which its shape is derived.

## Retention

`RetentionOrder` is declared, carried, stored, and acknowledged. No pruning
behavior exists in this crate, and the current Mirror runtime does not enforce
the order. That boundary is deliberate: vocabulary can exist before the
mechanism that acts on it.

## Authority and projection

| Path | Responsibility |
|---|---|
| `ethos/signal.ethos` | sole authored Interface authority |
| `src/bootstrap_manifest.rs` | already-minted authority and declaration seats |
| `build.rs` | verifies the checked-in Rust projection against the authored Ethos |
| `src/generated/signal.rs` | checked encoded Rust projection |
| `src/lib.rs` | Signal archive and byte-carrier behavior |
| `examples/canonical.datom` | exact readable witnesses for every root variant, in current Datom text |

The build accepts the exact Cargo-published Ethos sources from `signal-mirror`
and `signal-standard`, verifies them against the Rust constants compiled from
those same revisions, then publishes this repository's `ethos/` directory for
its consumers.
