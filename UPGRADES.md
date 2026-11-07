# Upgrades

## 4.0.0 → 4.1.0

First Datom/Protos 0.33 adoption. Authored Ethos, library, build script and
canonical examples remain unchanged. Thirteen generated aliases become tuple
structs with Datom derives. Existing binary fixtures require no constructor
changes. ReaderBudget construction is checked; the canonical comparison uses
compact() to retain exact single-line equality, including exhaustive matches
over the five Query and six Response roots. Raw witnesses independently use
textualize(), not compact normalization.

Selected cohort: Datom eabead4 / Protos 3c7ade3 (0.33.0), Signal 7afb68b
(10.3.0), ordinary Mirror d0c5d1b (5.1.0), and Ethos Zero 5a5daca (19.2.0).
Registry packages/checksums are retained. The existing app's old pinned
contract/features remain untouched and uncertified.

Historical d91f8e1 source with its own lock/tools and the candidate cover
all eleven existing canonical root values. Archives and Signal::bytes()
frames are byte-equal 11/11; local restores and both directions of typed
binary/text cross-reads pass. Raw text is equal 7/11; balanced-string/token
inspection verifies layout-only differences for the other four samples.
The binary MemoryVersionedLog and malformed-byte regression tests remain.
These are bounded root/value samples, not exhaustive source-API, storage,
runtime or transport-framing proof.

Checks cover x86_64-linux only; other declared systems are omitted.
No app or runtime rollout is performed by this source release.
