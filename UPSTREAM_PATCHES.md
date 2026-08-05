# Upstream patches

This branch is based on upstream Reth `v2.0.0`
(`eb4c15e5e36d8776d46629beae4c0a69af7ab04f`).

## RocksDB synchronous writes

- **Scope:** `reth-provider` RocksDB write transactions and batches.
- **Source:** exact backport of upstream Reth commit
  `3a136fc8c38221e060cbc31ef5c5fa345cf0e17a` (PR #23603).
- **Purpose:** set `WriteOptions::sync(true)` for transactions, explicit batches, auto-committed
  batches, and final batch commits so a successful commit is durable across a host crash.
- **Compatibility:** this changes write durability only and retains the Reth 2.0.0 / REVM 36
  dependency family.
- **Removal condition:** remove this branch when the selected upstream Reth release contains PR
  #23603 or an equivalent synchronous-write implementation.
