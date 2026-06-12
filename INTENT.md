# INTENT — meta-signal-mirror

`meta-signal-mirror` is the meta policy wire contract of the mirror triad —
the owner-only configuration and policy vocabulary for the payload-blind
sema version-control mirror daemon.

Psyche intent, quoted from the Spirit store:

Spirit `0yx5` (Decision, High): [The sema version-control remote is a
dedicated mirror component triad: mirror, signal-mirror, and
meta-signal-mirror. One payload-blind append-ingest mirror daemon on the
ouranos tailnet host serves every component store: it validates sequence
continuity and expected head, deduplicates idempotently, fsyncs before
acknowledging, and carries retention and privacy policy behind its meta
signal. The mirror daemon's own durable state is a sema-engine store. The
psyche authorizes creating the three new repositories.]

Spirit `rj9y` (Decision, High): [Cross-host component transport for the
version-control mirror is a tailnet-bound TCP listener in triad-runtime,
reusing the length-prefixed frame codec, with peer identity as a typed closed
sum distinguishing kernel-vouched Unix-socket peers from tailnet TCP peers.
Ssh-forwarded sockets are rejected as the transport shape.]

Spirit `29pb` (Constraint, High): [Component Sema databases, the daemon
durable state, must be backed up to a server atomically, and state loss is
unacceptable. Pursue native version-controlled component databases rather
than treating the store as an opaque binary blob. Mechanism is under design
and Dolt-informed, with the strict-typed hard-migration-per-schema-change
shape as the core constraint to solve.]

Spirit `x0ja` (Constraint, High): [One consistent cryptographic basis spans
the entire version-control and backup system: blake3 for all content
addressing and criome BLS for signing and attesting history. No component
diverges in hash function or crypto.]

Load-bearing consequences for this contract:

*Retention policy is a typed placeholder in this cut.* `0yx5` puts retention
and privacy policy behind the meta signal; this contract names and stores the
typed `RetentionOrder`, but the mirror daemon does not yet enforce it —
enforcement is deferred by decision to a later cut.

*Criome BLS signing/attestation (`x0ja`) is deferred by decision* to a later
cut; the meta surface carries no signature or attestation vocabulary yet. Per
`rj9y` this cut is tailnet-trusted with no per-request authentication; the
meta surface stays Unix-owner-only and is never reachable over TCP.

*Store registration is meta authority.* A component store ships history only
after its owner registers the store name here; the working contract has no
registration verb.
