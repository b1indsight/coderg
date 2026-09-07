# Case-insensitive query budgets on vLLM

The accepted decision, full context, implementation constraints, and follow-up
memory estimates are recorded in the [Chinese decision record](../../docs/decisions/2026-09-07-case-insensitive-index-budget.md).

The selected defaults are **128 literal alternatives per fragment and 128
distinct index keys per query**, replacing 64 / 256. They gave the best aggregate
latency among the six configurations confirmed with production release builds.
The gain is small: about 0.7% for default searches and 1.0% with `--no-refresh`.
64 / 128 is also close. This experiment identifies a useful operating range,
not a universal or uniquely optimal pair of integers.

## Environment and method

- Apple M5, 10 logical CPUs, 24 GiB RAM, macOS; Rust 1.94.0.
- vLLM commit `569adb5a9780f9c02d22a6b29826acf711512356`.
- 6,835 indexed text files, 86,384,180 source bytes (82.38 MiB).
- Existing index directory: 69,627,944 bytes (66.40 MiB). Every index file's
  SHA-256 was unchanged after the experiments; no index rebuild was needed.
- Normal filename / line number / matching-line output, redirected to
  `/dev/null`. This is not a files-only benchmark. Filesystem caches were warm.
- Coarse grid: variants `{4, 8, 16, 32, 64, 128}` crossed with keys
  `{8, 16, 32, 64, 128, 256, 512}`: 42 configurations.
- Refinement: variants `{24, 48, 96}` crossed with keys
  `{64, 96, 128, 192, 256}`, plus variants `{32, 64, 128}` crossed with keys
  `{96, 192}`: 21 additional configurations. Four coarse configurations were
  repeated as controls, for 63 distinct configurations overall.
- Each sweep used eight selection queries, two warmups and seven randomized,
  interleaved timings per configuration / query, with `--no-refresh`.
- The sweep used an isolated release build with temporary environment-based
  limits. Diagnostics recorded actual keys and candidate files outside timed
  runs. These environment controls are not part of the shipped CLI.
- Six finalists were compiled again with real constants and no instrumentation.
  Each had three warmups and 31 randomized, interleaved timings in both default
  and `--no-refresh` modes, across eight selection queries, six additional
  validation queries and three case-sensitive controls.
- Three `/usr/bin/time -l` peak-RSS measurements per finalist / query in default
  mode. `rg` was measured alongside the finalists.
- Complete normalized output was checked against `rg` for every sweep query /
  configuration and both modes of every finalist / query. Only correctness
  checks used `rg --sort path`; timed `rg` retained parallel traversal.
- The key limit also caps the number of planned strategy groups, as in the
  implementation. The experiment therefore tunes these coupled limits.

## Confirmed results

Latency is the equal-weight arithmetic mean of per-query medians across the
14 case-insensitive queries. RSS average is the mean of per-query median peak
RSS; maximum is the largest peak RSS observed across those queries and repeats.

| Variants / keys | Default ms | No-refresh ms | RSS average MiB | RSS maximum MiB | Actual keys average |
|---|---:|---:|---:|---:|---:|
| 32 / 64 | 45.719 | 28.553 | 24.70 | 27.41 | 41.4 |
| 32 / 128 | 45.521 | 28.475 | 25.17 | 28.81 | 59.9 |
| 48 / 128 | 44.050 | 26.897 | 25.47 | 29.11 | 69.0 |
| 64 / 128 | 42.821 | 25.573 | 25.05 | 28.34 | 77.1 |
| **128 / 128** | **42.614** | **25.484** | **24.82** | **27.80** | **83.4** |
| 64 / 256, previous | 42.925 | 25.737 | 25.41 | 29.55 | 90.6 |

For 128 / 128, the selection-query averages were 41.190 / 24.058 ms
(default / no-refresh), and the additional-query averages were 44.512 /
27.385 ms. The corresponding previous-default averages were 41.575 / 24.452 ms
and 44.726 / 27.450 ms.

The mean of per-query p95 latencies was 44.03 ms default and 27.25 ms
no-refresh, compared with 44.82 and 27.30 ms previously. The three case-sensitive
controls showed no material regression; their strategy keys and candidates were
unchanged. Individual case-insensitive queries can still get slightly slower.

Paired-round bootstrap resampling, 2,000 repetitions, gave 95% intervals for the
aggregate improvement of 0.29%–1.09% default and 0.22%–1.39% no-refresh. These
intervals measure timing noise for this fixed workload, not uncertainty across
repositories or query populations. Final selection considered the additional
queries, so they are not an untouched test set for the selected defaults.

## Why not smaller or larger budgets?

- At 8 or 16 variants, four of the eight selection queries fell back to full
  scans even with a 256-key budget. Their aggregate no-refresh latency was
  roughly 40 ms, versus roughly 24 ms for the better configurations.
- 32 and 48 variants looked competitive on the selection queries, but the
  additional queries exposed regressions. Searching `class … Attention` took
  about 44 ms at 32 or 48 variants, versus about 37–38 ms at 64 or 128.
- The three-exception-type query took about 82 ms at 32 / 128, versus 61 ms at
  128 / 128. The tensor-allocation query took about 60 ms at 48 / 128, versus
  49 ms at 128 / 128. These are ordinary code-search patterns.
- Larger key budgets did not help the aggregate sweep: at 64 variants,
  128 / 256 / 512 keys produced 24.15 / 24.30 / 24.48 ms respectively.
- For the long fixed identifier, the selected defaults read 126 keys instead
  of 250, while candidates increased from 254 to 505 files. Despite reading
  more candidate files, no-refresh latency fell from 19.84 to 18.16 ms and
  median peak RSS from 29.52 to 26.05 MiB. More precise filtering did not repay
  all of its posting-list processing cost in this case.

Halving the hard key ceiling does not halve typical work or memory: average
actual keys decreased from 90.6 to 83.4, and average peak RSS from 25.41 to
24.82 MiB. Complete alternative unions remain mandatory; an over-budget union
is skipped as a whole, preserving prior safe filters.

## Workload

All selection and additional queries use `-i`. The long fixed identifier also
uses `-F`.

Selection queries:

```text
\basyncmock\b
\b(cuda|rocm)\b
\bSamplingParams\b
logger\.(warning|error)\(
get_tensor_model_parallel_world_size
\b(kv_cache|block_size)\b
\b(TODO|FIXME)\b
\b(VLLM|CUDA)_[A-Z0-9_]+\b
```

Additional validation queries:

```text
class[ \t]+\w*Attention\b
def[ \t]+forward\b
\bmax_(model_len|num_seqs)\b
\b(ValueError|RuntimeError|NotImplementedError)\b
\b(torch|numpy)\.(empty|zeros|ones)\b
async[ \t]+def[ \t]+\w+
```

Case-sensitive controls repeat the long fixed identifier, `SamplingParams`, and
the `forward` definition query without `-i`.

## Artifacts and validation

The [evidence archive](data/vllm-case-budget-2026-09-07/manifest.json) preserves
the original timing samples, RSS measurements, protocols, binary and index
hashes, per-query diagnostics, summaries, historical scripts, and source diffs.
The original local experiment directory was
`/private/tmp/coderg-budget-sweep.XV9mfy`; the archived evidence does not depend
on that directory remaining available. Historical scripts retain their original
absolute paths and need path updates before reuse.

After applying 128 / 128, all 20 existing tests passed, including Unicode case
folding, complete alternative unions, and hard-budget exhaustion. Clippy passed
with warnings denied. This experiment does not cover cold caches, other
repositories, or variants above 128, and does not establish a global optimum.

## Follow-up: 128 / 256 and p95

A follow-up compared 128 / 128 directly with 128 / 256, changing only
`MAX_INDEX_LOOKUPS` in an isolated production release build. The current
workspace binary was the 128 / 128 baseline. The same 14 case-insensitive
queries received five warmups and 101 randomized interleaved timings per
configuration / mode / query, with randomized query order. The per-query p95
is the 96th smallest of 101 samples. Each table cell remains the mean of
per-query medians or p95s, matching the main report's metric definition.

| Mode | Metric | 128 / 128 ms | 128 / 256 ms | Change with 256 keys |
|---|---|---:|---:|---:|
| Default | Median mean | 42.123 | 42.360 | +0.237 ms (+0.56%) |
| Default | p95 mean | 45.886 | 45.845 | -0.041 ms (-0.09%) |
| No-refresh | Median mean | 25.259 | 25.445 | +0.186 ms (+0.74%) |
| No-refresh | p95 mean | 27.113 | 27.171 | +0.059 ms (+0.22%) |

The 256-key configuration was slightly slower in median latency. Paired-round
bootstrap 95% intervals for relative slowdown were +0.28% to +0.84% default
and +0.47% to +1.07% no-refresh. The corresponding p95 intervals were -2.67%
to +3.23% and -3.84% to +2.54%, so this run does not establish a p95 difference.
These are measurement intervals for this fixed workload, using 2,000 resamples.

The earlier 31-round comparison of 64 / 256 against 128 / 128 observed a
default p95-mean reduction of 0.792 ms (1.77%) and a no-refresh reduction of
0.049 ms (0.18%). Reanalyzing those samples gave improvement intervals of
-9.00% to +7.47% and -1.09% to +2.20%. Thus the earlier p95 point estimates
also do not establish a stable tail-latency improvement. Absolute values from
the two runs should not be treated as a simultaneous comparison.

Some individual queries show more noticeable costs with 256 keys:

| Query | Default median change | Default p95 change | Keys, 128 → 256 budget | Candidate files, 128 → 256 budget |
|---|---:|---:|---:|---:|
| Attention class definition | +1.538 ms | +1.628 ms | 124 → 164 | 976 → 973 |
| Long fixed identifier | +1.209 ms | +1.038 ms | 126 → 256 | 505 → 254 |
| SamplingParams | +1.174 ms | +1.060 ms | 128 → 201 | 529 → 384 |

Average actual keys increased from 83.4 to 103.9 (+24.5%), while average
candidate files decreased only from 1,540.5 to 1,510.9 (-1.9%). The Attention
query is a particularly clear example: 40 additional keys eliminated only
three candidate files. The 128 / 128 defaults remain unchanged.

For completeness, the p95 of all samples mixed with equal query frequency was
59.471 → 59.587 ms default and 43.161 → 42.686 ms no-refresh. This is a
different statistic from the mean of each query's p95 reported above, and
does not represent a measured real-world query frequency distribution.

All 56 full-output comparisons matched the prior `rg` oracle. Index file
hashes remained unchanged. The [archived samples](data/vllm-case-budget-2026-09-07/key256/results.json)
and [summary](data/vllm-case-budget-2026-09-07/key256/summary.json) preserve the
binary hashes, confidence intervals, and per-query results. The historical
build and analysis scripts are in the archive's `scripts/` directory.
