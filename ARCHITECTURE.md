# meta-signal-mirror — Architecture

`meta-signal-mirror` owns one thing: the vocabulary by which an owner
configures a Mirror and governs its stores. It contains no daemon, actors,
persistence, pruning machinery, or transport server.

## Semantic center

This Interface makes owner authority legible without copying the ordinary
Mirror vocabulary. `StoreName` comes from `signal-mirror`; `SocketPath` and
`NetworkEndpoint` come from `signal`. The relations therefore share
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
| `ethos/signal.ethos` | sole authored schema authority |
| `build.rs` | asserts the checked-in Rust projection against a fresh `ethos-zero` generation |
| `src/generated/signal.rs` | checked Rust projection |
| `src/lib.rs` | module roots and the re-exported portable frame |
| `examples/canonical.datom` | every request and reply root as encoded Datom text |
| `tests/canonical.rs` | actualizes every canonical line and renders it back |

The portable rkyv frame — `Signal`, `Signalizable`, `ByteViewable`,
`Restorable` — comes from `signal` and is re-exported from `src/lib.rs`. This
crate holds no frame of its own. Shared identities (`StoreName`, `SocketPath`,
`NetworkEndpoint`) are imported from the pinned `signal-mirror` and `signal`
revisions, never restated.

## Verification

`tests/canonical.rs` actualizes each line of `examples/canonical.datom` through
the codec, renders the value back, and requires the rendering to reproduce the
authored line exactly. Root coverage is asserted against exhaustive matches
over `Query` and `Response`, so a root added to the Ethos source cannot compile
until it is given a canonical line. `tests/round_trip.rs` carries a request and
a reply across fresh peer bytes and proves malformed bytes are refused.
`tests/dependency_boundary.rs` proves the default graph pulls no retired codec
or generator.
