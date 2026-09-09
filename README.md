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
- Git tree manifests let commits promote an existing working-tree overlay and
  let rollbacks reuse a previously cached tree index.

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
The [latest project benchmark](benches/results/main-projects-2026-09-09.md) measures
build time, peak memory, and query latency against the previous main branch on
frozen vLLM, viberwhisper, and agentflow snapshots.

The index stores immutable lookup/postings pairs under `segments/`. Every
document points to the segment containing its current version, so postings
from older versions are ignored. A normal edit creates one batched delta;
creating a Git commit for already-indexed content only updates the manifest.
Returning to a cached Git tree switches manifests without rebuilding. Large
uncached changes and excessive segment counts fall back to a fresh base build.
Git HEAD, tree, and worktree status are read in-process through libgit2. When
HEAD and tree are unchanged, a file-metadata snapshot check avoids a full Git
status walk. Clean cached trees reuse existing segments and refresh their
metadata snapshots; small changes update only the affected file contents.

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

The [binary manifest benchmark](benches/results/chromium-binary-manifest-2026-09-09.md)
reduces Chromium manifest read and decode time from 205.08 ms to 30.76 ms, and
the binary file occupies 110.23 MiB. Current release no-refresh search for
`MAX_FILE_SIZE` takes 59.96 ms; default search takes 1.86 s because it still
checks the full source snapshot.

The [index-build memory optimization](docs/index-build-memory-optimization.md)
explains the construction data structures and their measured memory savings.
The [refresh-path optimization](docs/refresh-path-optimization.md) explains
snapshot batching, manifest parsing, and the small-repository traversal policy,
including profiling evidence, threshold selection, and comparisons with rg.

The repository includes a process-level benchmark that compares the release
build of `coderg` with `rg`. It checks that both tools return the same files,
then reports minimum, median, p95, and mean wall-clock latency.

```sh
# Generate a deterministic 4 MiB source corpus and run 30 timed iterations.
cargo bench --bench compare_rg

# Change generated corpus size and sample count.
cargo bench --bench compare_rg -- \
  --files 1000 --kib-per-file 32 --iterations 50

# Benchmark an existing repository and fixed-string query.
cargo bench --bench compare_rg -- \
  --root /path/to/repository --query AsyncMock --iterations 50

# Machine-readable output for CI or plotting.
cargo bench --bench compare_rg -- --iterations 50 --json
```

The benchmark measures index construction separately, then compares indexed
search without refresh, indexed search with the normal metadata/Git freshness
check, and ripgrep. Generated-corpus runs use a temporary Git repository and
also report single-file incremental refresh, commit promotion without
reindexing, and cached-tree rollback latency. Set `CODERG_BIN` to benchmark a
specific `coderg` binary.

For matching-line comparisons and a persistent JSON report:

```sh
cargo bench --bench compare_rg -- \
  --root /path/to/repository --query MAX_FILE_SIZE --lines \
  --iterations 15 --warmup 3 --output /tmp/coderg-bench-001.json
```

The report includes the query, output mode, warmup count, individual samples,
index build time, and minimum/median/p95/mean search latency. Choose a new output
file with an existing parent directory. The benchmark uses a temporary index
and compares against `rg --hidden --no-config`; keep the source tree unchanged
throughout the run. Searches measure warm-cache process latency.
