# meta-signal-mirror agent notes

Read `/home/li/primary/AGENTS.md` first, then this repo's `INTENT.md` and
`ARCHITECTURE.md`.

`meta-signal-mirror` is the meta policy wire contract of the mirror triad:
owner-only configuration, store registration/retirement, and the typed
retention placeholder for the sema version-control mirror daemon.

Before editing, read `/home/li/primary/skills/contract-repo.md` and
`/home/li/primary/skills/component-triad.md`.

Load-bearing rules for this repo:

- Wire-only: no runtime, no actors, no tokio, no enforcement logic.
- Edit `schema/lib.schema` and regenerate
  (`META_SIGNAL_MIRROR_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`); never
  hand-edit `src/schema/lib.rs`.
- Retention is a typed placeholder: named and stored, not enforced —
  enforcement is deferred by decision (`INTENT.md`). Do not present the
  placeholder as enforced policy.
- The meta surface is Unix-owner-only; never add a TCP-reachable meta verb.
