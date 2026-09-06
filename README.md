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

The default index directory is `<root>/.coderg-index`. Normal `.gitignore`,
`.ignore`, and global Git ignore rules are respected. Hidden source files are
included, while `.git`, `.coderg-index`, ignored files, symlinks, and files
whose first 8 KiB contain NUL bytes are excluded.

The index stores immutable lookup/postings pairs under `segments/`. Every
document points to the segment containing its current version, so postings
from older versions are ignored. A normal edit creates one batched delta;
creating a Git commit for already-indexed content only updates the manifest.
Returning to a cached Git tree switches manifests without rebuilding. Large
uncached changes and excessive segment counts fall back to a fresh base build.
Git HEAD, tree, and worktree status are read in-process through libgit2. A clean
cached tree is selected before the filesystem walker runs; dirty trees use the
changed paths reported by Git as a small overlay over the cached base.

Patterns without a safe literal of at least three bytes (including current
case-insensitive searches) fall back to scanning all indexed text files. This
keeps behavior correct while reserving indexed acceleration for queries that
can use it safely.

## Benchmark against ripgrep

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
