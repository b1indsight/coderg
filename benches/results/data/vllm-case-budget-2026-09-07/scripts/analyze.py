import json
import random
import statistics
from common import *

data = json.loads((OUT / 'confirmation-results.json').read_text())
rows = data['queries']
configs = [config_name(c['variants'], c['keys']) for c in data['finalists']]
BASELINE = 'v64-k256'
rng = random.Random(2026090703)

def aggregate(selected, name):
    result = {}
    for mode in ['default', 'norefresh']:
        variant = f'{name}/{mode}'
        baseline = f'{BASELINE}/{mode}'
        median = statistics.mean(row['variants'][variant]['median_ms'] for row in selected)
        base_median = statistics.mean(row['variants'][baseline]['median_ms'] for row in selected)
        result[mode] = dict(avg_median_ms=median,
            avg_p95_ms=statistics.mean(row['variants'][variant]['p95_ms'] for row in selected),
            improvement_pct=100 * (base_median - median) / base_median)
        if name == BASELINE:
            result[mode]['improvement_bootstrap_95pct_ci'] = [0, 0]
            continue
        draws = []
        for _ in range(2000):
            sums = [0.0, 0.0]
            for row in selected:
                actual = row['variants'][variant]['samples_ms']
                base = row['variants'][baseline]['samples_ms']
                indexes = [rng.randrange(len(actual)) for _ in actual]
                sums[0] += statistics.median(actual[i] for i in indexes)
                sums[1] += statistics.median(base[i] for i in indexes)
            draws.append(100 * (sums[1] - sums[0]) / sums[1])
        draws.sort()
        result[mode]['improvement_bootstrap_95pct_ci'] = [draws[49], draws[1949]]
    result['avg_keys_loaded'] = statistics.mean(row['trace'][name]['keys_loaded'] for row in selected)
    result['avg_candidate_files'] = statistics.mean(row['trace'][name]['candidate_files'] for row in selected)
    result['full_scan_queries'] = sum(row['trace'][name]['full_scan'] for row in selected)
    rss = [[sample['peak_rss_bytes'] / 1048576 for sample in row['memory'][f'{name}/default']] for row in selected]
    result['rss_avg_query_median_mib'] = statistics.mean(statistics.median(values) for values in rss)
    result['rss_max_observed_mib'] = max(max(values) for values in rss)
    return result

summary = {}
for split in ['train', 'holdout', 'all_i', 'control']:
    selected = [row for row in rows if row['query']['split'] == split or (split == 'all_i' and row['query']['split'] != 'control')]
    summary[split] = {name: aggregate(selected, name) for name in configs}
    summary[split]['rg'] = dict(avg_median_ms=statistics.mean(row['variants']['rg']['median_ms'] for row in selected),
        rss_avg_query_median_mib=statistics.mean(statistics.median(s['peak_rss_bytes'] / 1048576 for s in row['memory']['rg']) for row in selected))
    print(f'{split}:', flush=True)
    for name in configs:
        row = summary[split][name]
        print(f'  {name}: default {row["default"]["avg_median_ms"]:.3f} ms, no-refresh {row["norefresh"]["avg_median_ms"]:.3f} ms, RSS avg/max {row["rss_avg_query_median_mib"]:.2f}/{row["rss_max_observed_mib"]:.2f} MiB, keys {row["avg_keys_loaded"]:.1f}', flush=True)
save(OUT / 'summary.json', summary)
lines = ['# vLLM case-insensitive budget tuning', '',
         'All latency aggregates below are the equal-weight mean of each query\'s median. RSS average is the mean of per-query medians from three measurements; maximum is the largest observed run.', '',
         'Coarse sweep: 42 configurations. Local refinement: 21 additional configurations, with four repeated controls. Six real constant binaries: 31 randomized interleaved timings per mode/query, 3 warmups, 3 RSS measurements. Eight selection queries, six additional validation queries, three case-sensitive controls.', '',
         'Warm filesystem caches; normal matching-line output redirected to /dev/null. Each tested configuration was checked against the complete normalized rg output. Only correctness checks use rg --sort path. The pre-existing index remained byte-for-byte unchanged.', '',
         'The bootstrap intervals describe repeated-measurement noise for this fixed workload. They do not estimate uncertainty across repositories or query populations. Resampling preserves paired iteration rounds within each query; 2,000 bootstrap repetitions.', '']
for split in ['train', 'holdout', 'all_i', 'control']:
    lines += [f'## {split}', '', '| Variants / keys | Default ms | No-refresh ms | Default gain vs 64/256 | NR gain vs 64/256 | RSS avg / max MiB | Actual keys avg |',
              '|---|---:|---:|---:|---:|---:|---:|']
    for name in configs:
        row = summary[split][name]
        lines.append(f'| {name} | {row["default"]["avg_median_ms"]:.3f} | {row["norefresh"]["avg_median_ms"]:.3f} | {row["default"]["improvement_pct"]:.2f}% | {row["norefresh"]["improvement_pct"]:.2f}% | {row["rss_avg_query_median_mib"]:.2f} / {row["rss_max_observed_mib"]:.2f} | {row["avg_keys_loaded"]:.1f} |')
    lines += ['', 'Default/NR improvement 95% intervals:', '']
    for name in configs:
        row = summary[split][name]
        lines.append(f'- {name}: {row["default"]["improvement_bootstrap_95pct_ci"]}; {row["norefresh"]["improvement_bootstrap_95pct_ci"]}')
    lines.append('')
lines += ['## Per-query medians, default mode', '', '| Query | Split | ' + ' | '.join(configs) + ' | rg |',
          '|---|---|' + '---:|' * (len(configs) + 1)]
for row in rows:
    pattern = row['query']['pattern'].replace('|', r'\|')
    lines.append('| `' + pattern + '` | ' + row['query']['split'] + ' | ' + ' | '.join(f'{row["variants"][f"{name}/default"]["median_ms"]:.3f}' for name in configs) + f' | {row["variants"]["rg"]["median_ms"]:.3f} |')
lines += ['', '## Artifacts', '', '- [Summary and bootstrap intervals](summary.json)', '- [Full confirmation samples and diagnostics](confirmation-results.json)', '- [Coarse sweep](coarse-results.json)', '- [Local refinement](fine-results.json)', '- [Original protocol, including full query list](protocol.json)', '- [Refinement protocol](fine-protocol.json)', '- [Confirmation protocol and binary SHA-256](confirmation-protocol.json)', '- [Index files before/after verification](index-before.json)', '- [Snapshot/harness preparation](prepare.py), [sweep runner](sweep.py), [confirmation runner](confirm.py), [analysis](analyze.py)', '']
(OUT / 'report.md').write_text('\n'.join(lines))
