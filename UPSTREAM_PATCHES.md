# Upstream Patch Queue

This branch is based on Reth `eb4c15e5e36d8776d46629beae4c0a69af7ab04f`.

## Header transformation hooks

- **Scope:** `reth-network` and `reth-node-builder` only.
- **Purpose:** expose generic asynchronous hooks for transforming headers after download and before
  serving a header response.
- **Protocol ownership:** the hooks contain no DogeOS or Scroll rules. Callers own signature
  extraction, verification, persistence, and restoration.
- **Default behavior:** both hooks are no-ops unless explicitly configured.
- **Required by:** Feynman-and-later DogeOS historical sync, where the wire representation carries
  a signed-header field that is not part of the canonical stored header.
- **Removal condition:** remove this patch once equivalent downloader and response transformation
  hooks are available in the selected upstream Reth release.
