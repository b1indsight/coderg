# Git commit snapshots and worktree overlays

Repositories with a committed HEAD index that tree as a baseline and index worktree
changes as an overlay. Non-Git and unborn repositories retain the existing refresh
policy. Stable document IDs are append-only within a cache epoch.

## Content identity and publication

Git blob IDs identify actual indexed bytes. Unchanged metadata can reuse the previous
identity; changed content is hashed with metadata checks before and after reading.
A new tree uses the prior baseline and Git diff to resolve changed entries. An indexed
worktree version whose blob ID matches the commit is promoted by reference; missing
versions are extracted from the worktree or Git object database. Partial commits keep
remaining changes in the worktree overlay. Checkout transformations and filters must
match actual bytes, not merely a clean Git status.

Initial builds cache checked small-file contents within the posting memory budget:
at most 32 MiB and one quarter of available posting capacity, with files up to 1 MiB
preferred. Other files retain the bounded streaming path. Directory tree lookups are
shared. This avoids reading cached content twice during identity checks and extraction.

The active CDRGMF02 manifest includes a publication identity and registry prefix
identity. Ordinary unchanged search maps records and does not load snapshot/state
sidecars. Full decoding and integrity verification are deferred until an update.
Refresh installs the verified view under the writer lock without a second search load.

## Retention and maintenance

The cache retains the 8 most recently used distinct Git trees. Identical-tree commits
share a snapshot; a ninth tree evicts the oldest. Evicted versions may require extraction
when revisited. No unlimited history or background compact-history command is enabled.
This bounds snapshot count, not a fixed byte total; the append-only path registry can
still grow and an explicit index rebuild starts a new epoch.

Automatic maintenance is only checked when that layer extracts new content. Publishing
already indexed content or visiting a cached commit does not trigger rewriting.
An overlay exceeding 8 segments merges up to 32 inputs. Baseline deltas accumulate by
bytes: if the largest segment is below 32 MiB, other segments must reach 8 MiB; otherwise
they must reach max(8 MiB, 25% of the largest segment). Consolidation can use several
bounded-fan-in batches; it is still synchronous. Old snapshots keep their referenced
segments, while unreferenced files are collected after publication.

Readers hold read.lock while opening immutable mmaps; Unix readers survive later unlink.
Other platforms defer segment deletion. Corrupt or missing optional state disables reuse
and falls back to extraction. Concurrent writers serialize publication, and an interrupted
sidecar/manifest publication is detected by identity mismatch. This is not a transactional
snapshot of a concurrently edited worktree; same-size/same-mtime replacements retain the
existing scanner limitation.

## Validation

Regression coverage includes partial commits, binary files, rename/deletion, subtree roots,
checkout filters, linked worktrees, cache corruption and eviction, mmap reader lifetime,
and dirty worktree changes. snapshot_workflow.py checks segment identity across promotion
and cached revisits, including alternating commits on two branches.

Final merge validation: [two repositories, 3 × 100 history updates and alternating branches](https://github.com/b1indsight/coderg/blob/5ca67a2470ac9e9e3b7cc08ad3f2b8115f68a170/benches/results/retained-merge-2026-09-21.md).
