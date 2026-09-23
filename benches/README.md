# Unified Rust benchmark suite

`cargo bench` runs the full suite in [full.json](full.json). The runner, corpus
preparation, synthetic history, measurement, verification and reporting are Rust.
External programs are the Cargo-built `coderg`, `rg`, Git, and `/usr/bin/time` for
RSS on macOS/Linux. Stable Rust is sufficient; there is no Python runtime dependency.

## Configure local corpus locations

Copy [sources.example.json](sources.example.json) to `benches/sources.local.json`
and replace the paths with existing local sources. This local file is ignored by
Git. Paths are relative to the package root, or absolute. `--sources FILE` selects
another map. Without a local map, the paths in `full.json` are used. Nothing is
downloaded automatically; missing sources or insufficient history fail preflight.

| Corpus | Pinned revision | Updates | History |
|---|---|---:|---|
| viberwhisper | `bc73d1c54089dad53a3f5585ed5210b188542817` | 100 | real first-parent |
| agentflow | `911ea1ee5ae444c466e568edf2402b6fd15d8aea` | 56 | real first-parent |
| whisper_cpp | `c44b60b8053bbf2a5c1e014f11323fb3f2485177` | 100 | real first-parent |
| vllm | `569adb5a9780f9c02d22a6b29826acf711512356` | 100 | real first-parent |
| chromium | `398630472335c10b9ca610a4d1b7888a040f702a` (archive label) | 100 | synthetic on real source |

Git corpora are cloned at the pinned SHA; local uncommitted changes are excluded.
The Chromium source must be an extracted archive without `.git`. Its regular-file
SHA-256 manifest is saved, then a disposable copy is committed and modified using
a fixed seed and deterministic commit identities/timestamps. The five-step cycle
edits one file, edits 1% of eligible files, adds a file, renames it, and deletes it.
These results describe a controlled workload, not Chromium's upstream history.

## Run

```sh
# Validate all five inputs and executable capabilities first.
cargo bench --bench suite -- --preflight

# Full five-corpus run, current coderg versus rg.
cargo bench

# Fast end-to-end validation of every scenario, using generated input.
cargo bench --bench suite -- --config benches/smoke.json

# Inspect the resolved config, or regenerate a report without rerunning workloads.
cargo bench --bench suite -- --list
cargo bench --bench suite -- --render target/benchmarks/full-RUN_ID
```

Use `--output NEW_DIRECTORY` to select a result directory and `--workspace DIR`
to choose where disposable clones/indexes are created. The default output is
`target/benchmarks/<suite>-<timestamp>`; workspace defaults to
`.cache/benchmark-workspaces`. Output directories must not already exist.
The current executable comes from `CARGO_BIN_EXE_coderg`, so Cargo selects and
builds it with the benchmark profile, including custom target directories.

The full preset is deliberately a long run: 16 timed search samples after two
warmups, three independent build/workflow/branch rounds, two history
replays, two separate RSS runs, and implementation-default threads and memory budget.
Both presets benchmark only the current Cargo-built executable using its default
search behavior. There is no older-binary option, no-refresh variant or thread
sweep. The harness removes `RAYON_NUM_THREADS` from child commands so the current
implementation selects its own default (currently four threads).
By default, up to two corpora run concurrently (`--jobs 2`), in bounded batches.
Each corpus keeps its scenarios sequential, with isolated fixtures and raw records.
Reports record the concurrency; these timings include resource contention. Use
`--jobs 1` for isolated measurements comparable to earlier serial runs. Failed
corpora retain their records and do not prevent other corpora from completing.
Do not run unrelated builds, tests, or benchmarks alongside measurement. The smoke
preset validates execution only; its small sample counts do not support a
performance conclusion. Smaller real-corpus runs can use a copy of `full.json`.
The harness never passes `--build-memory-mib`: every scenario uses the executable's
own default budget (currently 256 MiB). There is no memory-budget sweep or override
in the suite configuration. Build cases are labeled `default`.
Each corpus has one warmup build, three
timed builds and two independent RSS builds; other scenarios prepare their own indexes.
Missing CLI capabilities fail preflight rather than silently removing a scenario.

## Coverage and measurement

| Scenario | Workload |
|---|---|
| build | Fresh indexes, implementation-default threads and budget, latency, RSS, disk size, spill diagnostics |
| search | Literal, regex, OR, case folding, short/absent/broad queries; lines/files/counts; current default search versus rg |
| workflow | 1%/50% edits, promotion without rewriting segments, rollback/revisit, dirty/discard, untracked/add/rename/delete/ignore |
| branches | A1→B1→A2→B2, branch-specific output verification and commit promotion checks |
| history | Real or generated pinned commits, per-step updates, quarter-progress search checkpoints, rollback/revisit |
| manifest | Actual manifest search loading through the production View::open, with the production full-read fallback for legacy formats; no format conversion |

Search timing includes a fresh CLI launch, work, output to the null device, and
exit. Git operations, verification and state preparation are excluded. The
filesystem cache is warm; new processes and fresh indexes are not cold-cache
measurements. RSS comes from independent process invocations, including a replay
of the required prior states for stateful scenarios. Units are normalized to
bytes. Unsupported RSS platforms are recorded explicitly; macOS sandbox denial
is an error, never reported as zero memory use.

Search jobs are shuffled with a fixed seed, keeping all state-dependent operations
ordered. Every ordinary timing is serial. Both coderg and rg use their own default
thread counts. Correctness compares exit status and complete
output against `rg --hidden --no-config`, retaining duplicate lines and raw bytes.
Read-only search checkpoints also assert that the manifest did not change.

State changes are followed by the same probe query for coderg and rg, with engine
order shuffled per step. The coderg measurement includes synchronous refresh plus
search; the rg measurement scans the same working tree. These are end-to-end
step costs, not isolated internal refresh or query timings. Idle queries and the
full query checkpoints use the separate search scenario. Each recorded index
state also includes searchable file count and bytes to compare repository scales.
The step probe is the first configured query (an absent literal in the default
presets), which limits result-output cost; the search scenario covers the full
mix of common, broad, regex and output-mode queries.

Only main-supported workflows are included: there is no compact-history,
background maintenance, contention or explicit-compaction benchmark. A run always
builds the current checkout; it does not silently switch to main. To measure main,
apply these benchmark files in a main checkout and run cargo bench there. The
source branch, commit and dirty snapshot are recorded in run.json.

## Results and interpretation

- `run.json`: schema version, resolved config, binaries/source hashes, source Git
  identity, OS/architecture, tool versions, timing contract and completion status.
- `samples.jsonl`: raw values with corpus/scenario/case/variant/round/metric/unit;
  file order is execution order within each corpus. Delay and RSS samples have separate metrics.
- `events.jsonl`: exact measured commands, stderr, verification counts, commit
  identities, segment references and simulation operations.
- `summary.json` and `report.md`: N, min, median, mean, nearest-rank p95 and max,
  plus same-run differences of query and state-transition medians versus rg. They are not paired confidence intervals or a single overall score.
- `<corpus>-source-files.json`: regular-file SHA-256 manifest for archive input.

History steps are kept separate: their costs represent different changes, not
independent repetitions of one microbenchmark. Manifest records a single `load`
duration, with cases `search_view` and `search_full_fallback`. On Unix the view
maps the file, parses the header/segment directory, walks records and builds their
offset table; other platforms use the production buffered view. Measurements use
the actual index manifest with two warmups and 16 samples in the full preset.
They exclude CLI startup, reader locking, segment-file loading, freshness checks,
correctness validation and destruction, so they are not full index-load timings.
The actual artifact is validated by the complete decoder outside timing, and the file
must remain unchanged. Manifest rows are labeled `current`; loading uses the
current production code and current index artifacts. Binary and measured
source hashes are checked again before marking a run complete.

Errors after sampling begins preserve the raw records and leave the run marked
`incomplete`. Reports can be rendered again from those records; incomplete output
must not be treated as a successful full run. Abrupt termination can leave a
partial final JSONL line; remove that incomplete line in a copy before rendering.

## Generated output and retired entry points

Benchmark output is local and must not be committed. The default output under
`target/benchmarks/` and explicit output under `benches/results/` are ignored.
Old benchmark entry points, experimental drivers, and committed result artifacts
have been removed. Use the Rust suite for new measurements; historical evidence
remains available through Git history.

Detailed candidate/posting diagnostics require a stable production API.
The suite records existing CLI diagnostics without editing product code.
