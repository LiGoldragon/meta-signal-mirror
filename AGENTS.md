# meta-signal-mirror agent notes

Read `ARCHITECTURE.md` and `skills.md` before editing.

- `ethos/signal.ethos` is the sole schema authority.
- The source is a `Signal` root; request and reply seating is in the source itself.
- Import shared identities from the exact `signal-mirror` and
  `signal-standard` producers. Do not recreate them locally.
- No runtime, actors, persistence, transport server, or retention enforcement
  belongs here.
- Datom is the only text projection. Take the portable frame from `signal`.
- Work under exact-path claims and release them immediately after publication.

## Protos estate status

Stack: correct-new destination
Status: active component contract, on Ethos Zero, Protos, Datom, and Signal.
