# coderg

`coderg` is a local, indexed regular-expression search tool written in Rust.
It follows the design described in Cursor's
[Fast regex search](https://cursor.com/cn/blog/fast-regex-search) article:

- source files are mapped to numeric document IDs;
- deterministic sparse N-grams form a hashed inverted index;
- a sorted, memory-mapped lookup table points into delta-compressed posting
  lists stored in a separate file;
- regex literals select a small candidate set, then Rust's regex engine checks
  the original files, so hash collisions and index heuristics cannot create
  incorrect matches;
- immutable base and delta segments update only changed files;
- already-indexed changes can be committed without reindexing; rollbacks update
  from the current effective file snapshot;
- small bases rebuild at 8 MiB of accumulated deltas; bases of at least 32 MiB
  use B/M generations with a 25% merge threshold.

Sparse-gram weights now use a fixed English letter-frequency prior: rarer
letter pairs receive higher weights, with deterministic hash tie-breaking.
Index logic v5 requires rebuilding older indexes; default search does this
automatically, while `--no-refresh` rejects them. See the
[weighting rules and compatibility](docs/letter-frequency-weights.md).

## Install

```sh
cargo install --path .
```

The executable is named after this directory: `coderg`.

## Usage

```sh
# Build explicitly (search also builds automatically on first use)
coderg index /path/to/repository

# Regex and fixed-string searches
coderg search 'fn\s+main' /path/to/repository
coderg search -F 'MAX_FILE_SIZE' /path/to/repository
coderg search -i 'todo|fixme' /path/to/repository

# Familiar output modes
coderg search -l 'unsafe\s*\{' /path/to/repository
coderg search -c 'TODO' /path/to/repository

# Inspect index size
coderg stats /path/to/repository
```

Manifests now use a compact binary format. Existing JSON indexes remain readable;
rebuild with `coderg index` to use the new format immediately, or let the next
manifest update migrate it. Use `coderg stats /path/to/repository --json` to inspect
the records. See [format compatibility and the load benchmark](docs/manifest-format.md).

The default index directory is `<root>/.coderg-index`. Normal `.gitignore`,
`.ignore`, and global Git ignore rules are respected. Hidden source files are
included, while `.git`, `.coderg-index`, ignored files, symlinks, and files
whose first 8 KiB contain NUL bytes are excluded.

Index construction uses a **256 MiB working-buffer budget** by default, including
automatic rebuilds and incremental updates. Set `--build-memory-mib 256` on
`index` or `search` to override it (minimum: 16 MiB). Excess posting records spill
to temporary files and merge into the same index format. File metadata, Git
state, allocator overhead, and resident mappings are outside this buffer budget;
it is not a hard process RSS limit. See the [design and benchmark](docs/index-build-memory-budget.md).
The [build optimization overview](docs/index-build-evolution.md) compares the
original implementation with the current pipeline and summarizes measured gains.
The [project benchmark](benches/results/main-projects-2026-09-09.md) measures
build time, peak memory, and query latency against the previous main branch on
frozen vLLM, viberwhisper, and agentflow snapshots.
The [latest weighting benchmark](benches/results/identifier-word-or-2026-09-10.md)
compares 85 case-sensitive identifier, word, and OR queries over 31 rounds.
The default English prior performs about the same overall as measured Chromium
letter-pair weights, with gains and regressions on individual queries. On vLLM,
its index is 73.03 MiB versus 62.54 MiB for hash-only weights and 68.72 MiB for
Chromium pairs. See the [default-weight decision](docs/letter-frequency-weights.md#最新-benchmark-与默认方案).

The index stores immutable lookup/postings pairs under `segments/`. Every
document points to the segment containing its current version, so postings
from older versions are ignored. A normal edit creates one batched delta;
creating a Git commit for already-indexed content only updates the manifest.
When the base (B) is smaller than **32 MiB**, updates follow the original
append-only strategy: index changed files into a new delta segment and query
all referenced segments, ignoring obsolete document versions. When accumulated
delta lookup/postings files reach **8 MiB**, including the current update,
rebuild from the current source tree into one base. This replaces the old
8-segment and large-change rebuild conditions. B itself and unreferenced files
are excluded; obsolete postings in referenced delta segments still count.
Unchanged searches leave existing multi-segment indexes alone.
Small updates, threshold rebuilds, and Git-only advances use cache publication:
flush and atomic replacement, without `sync_all`. After a system crash this cache
may need rebuilding. If a rebuild grows B to 32 MiB, the new base and manifest
are synchronized before publication.

At **32 MiB or above**, the first segment is the stable base (B); all later
segments form the middle incremental generation (M). Normal edits and Git rollbacks update M from
the complete current file snapshot. Git identity is read through libgit2;
refresh no longer switches cached tree manifests or rebuilds because of a
segment-count/changed-file threshold.

Dead middle-segment references are pruned. Above eight live M segments, a
refresh can merge up to four similarly sized inputs totaling at most 32 MiB.
Compaction streams existing postings and filters obsolete document versions.
For these larger bases, when M reaches `max(25% of B, 8 MiB)`, a content refresh automatically merges
B and M into a new base. This runs synchronously and can make that refresh
slower. Mode selection uses B's lookup/postings bytes before the update, not
source size or B + M. The next update uses the resulting B's size to select its
mode. The manual command consolidates small indexes into B and merges M segments
for larger indexes:

```sh
coderg compact /path/to/repository --dry-run --json
coderg compact /path/to/repository
```

Writers serialize through an OS file lock and atomically replace the manifest.
Unchanged searches read immutable snapshots without that lock. Old segment
files remain on disk pending garbage collection. See the [final maintenance
design](docs/generational-index-refresh.md) for rollback examples, merge budgets,
publication semantics, and limitations. The query algorithm still reads only
relevant postings from each segment and filters obsolete document versions;
physical compaction rewrites all participating index records.

`compact --full` is no longer supported; use `index` to rebuild from source.
Default `stats` reports base/middle bytes, segment counts, `generational`, and
`maintenance due`. Below 32 MiB, the middle fields represent accumulated deltas
and the threshold field means the 8 MiB rebuild threshold. `index bytes` counts
only the current manifest and its referenced segments. `stats --json` exports
the full manifest; `compact --json` exports the selected/merged inputs.

The latest [three-size benchmark](benches/results/size-tiers-2026-09-10.md)
replays 100 real commits each from viberwhisper, whisper.cpp, and vLLM. It reports
ordinary updates, rebuilds, M merges, and B+M merges separately, with medians,
p95, B/M sizes, and independent rg query comparisons. Earlier experiments are
indexed in the design document; the per-update single-base trials are superseded.

The [refresh-path benchmark](benches/results/vllm-refresh-2026-09-07.md)
measures per-worker snapshot batches, parallel path sorting, and faster manifest
parsing, including repeated searches and Git state transitions. Refresh still
checks the complete file tree on every default search.
When the previous snapshot has at most 512 files, refresh collects and sorts
metadata on the calling thread to avoid parallel walker startup and shutdown
costs. Larger snapshots and initial builds use the parallel walker; candidate
matching continues to use Rayon. The previous file count is only a scheduling
hint, so newly added files are still discovered by a complete walk.
The [small-repository thread benchmark](benches/results/small-repo-threads-2026-09-07.md)
measures this policy: default searches improved by about 23–25% on the two
small real repositories, while large-repository aggregates remained within
the measured uncertainty intervals.
The [broader benchmark](benches/results/refresh-matrix-2026-09-07.md) covers
three real repositories and four generated corpora, with per-query comparisons
against the previous build and rg, memory measurements, and regression checks.

The [implemented optimizations and strategies](docs/implemented-optimizations.md)
document describes the current pipeline, storage layout, query budgets, refresh
paths, fallback behavior, and known limitations, with links to the implementation.
The [remaining design gaps with Cursor](docs/cursor-design-gaps.md) distinguish
documented algorithm differences from engineering follow-ups and outline the
next experiments.

Case-insensitive searches reuse the same index by looking up bounded Unicode
case variants. Each extracted fragment has at most 128 alternatives, and a query
reads at most 128 distinct index keys. Complete alternative lists are unioned;
required fragments are intersected. When the lookup budget is exhausted, prior
complete filters remain usable. Patterns without a safe literal of at least
three bytes fall back to scanning all indexed text files.

Search matches each LF-delimited line independently, like default `rg` searches.
Patterns such as `foo\s+bar` and `(?s)foo.*bar` cannot span lines. Empty files and
the position after a final newline do not produce matching lines. CR bytes are
preserved, and each matching line is reported or counted once. `-l` stops at the
first matching line in each file. Multiline search is not currently supported.
The [line matching benchmark](benches/results/line-matching-2026-09-09.md)
records vLLM output parity and the performance impact of this search path.

The [decision record](docs/decisions/2026-09-07-case-insensitive-index-budget.md)
documents the budget definitions, rationale, measurements, and limitations.
The [vLLM experiment report](benches/results/vllm-case-budget-2026-09-07.md)
includes the detailed comparisons and archived evidence.

## Benchmark against ripgrep

See the [benchmark index](benches/results/README.md) for current results and the
status of earlier experiments. Historical measurements refer to their archived binaries.

The [binary manifest benchmark](benches/results/chromium-binary-manifest-2026-09-09.md)
reduces Chromium manifest read and decode time from 205.08 ms to 30.76 ms, and
the binary file occupies 110.23 MiB. In that binary-manifest baseline, no-refresh search for
`MAX_FILE_SIZE` took 59.96 ms; default search took 1.86 s because it still
checks the full source snapshot.

The earlier [JSON-manifest Chromium benchmark](benches/results/chromium-max-file-size-2026-09-09.md)
searched `MAX_FILE_SIZE` across 461,882 indexed text files (2.83 GiB).
After warmup, 15 measured runs gave these median process times:

| Mode | Median | Speedup over rg |
|---|---:|---:|
| coderg, default freshness check | 2.055 s | 3.86× |
| coderg, `--no-refresh` | 263.96 ms | 30.01× |
| rg | 7.923 s | 1.00× |

All modes returned the same 98 files and 114 matching lines. The official
source snapshot excludes separately fetched DEPS dependencies and has no Git
metadata. Initial indexing took 65.24 s and produced a 1.95 GiB index; these
costs are separate from search timing. These are warmed repeated searches,
not cold-start measurements.

Before the binary manifest change,
[search profiling](benches/results/chromium-search-profile-2026-09-09.md)
attributed about 231 ms to loading the 208 MiB JSON manifest, including 197 ms
of deserialization. Candidate filtering takes 0.54 ms; reading and matching
the 102 candidates takes 4.95 ms. These internal stage times do not measure
the end-to-end latency of a persistent service, which has not been tested.
Default refresh separately spends about 1.75 s checking the source snapshot.

Earlier JSON manifest profiles show how costs depend on repository size and query selectivity:

| Profiled query | Manifest read + parse | Release no-refresh | rg |
|---|---:|---:|---:|
| vLLM `^class` | 3.18 ms | 30.75 ms | 77.51 ms |
| viberwhisper `^impl` | 0.086 ms | 3.83 ms | 6.27 ms |
| agentflow `^class` | 0.078 ms | 3.68 ms | 6.37 ms |

These follow-up measurements use 5 warmup rounds and 31 measured rounds,
with identical matching output verified against rg. See the
[vLLM profile](benches/results/vllm-no-refresh-profile-2026-09-09.md) and
[small-repository profiles](benches/results/small-no-refresh-profile-2026-09-09.md)
for stage timings, commands, and raw samples. Each row measures one query,
not an aggregate across a query suite.

The [index-build memory optimization](docs/index-build-memory-optimization.md)
explains the construction data structures and their measured memory savings.
The [refresh-path optimization](docs/refresh-path-optimization.md) explains
snapshot batching, manifest parsing, and the small-repository traversal policy,
including profiling evidence, threshold selection, and comparisons with rg.

The unified Rust suite runs through `cargo bench`. Configure the five local
corpora in `benches/sources.local.json` using
[the example map](benches/sources.example.json), then run:

```sh
cargo bench --bench suite -- --preflight
cargo bench
```

The default is the full real-corpus suite: viberwhisper, agentflow, whisper.cpp,
vLLM, and Chromium, comparing the current Cargo-built coderg with rg. Chromium
uses deterministic simulated commits on its pinned source archive. The suite
covers construction, query/output modes, Git workflows, history replay,
and manifest loading, using main-supported workflows. Only the current implementation is measured,
using its default threads, memory budget and search behavior, with rg as reference.
The harness runs up to two corpora concurrently; use `--jobs 1` for isolated timing.

```sh
# Small generated-corpus validation of all scenarios.
cargo bench --bench suite -- --config benches/smoke.json
```

Raw samples, provenance, JSON summaries and Markdown reports are written to a new
`target/benchmarks/<suite>-<timestamp>` directory. See the
[benchmark guide](benches/README.md) for configuration, measurement contracts,
report regeneration and the archived legacy scripts.
