"""Recompute tables from the stateful benchmark, using only the Python stdlib."""
import csv
import json
import math
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
MIB = 1024**2


def distribution(values):
    ordered = sorted(values)
    return {
        "median": statistics.median(ordered),
        "p95": ordered[math.ceil(len(ordered) * 0.95) - 1],
        "max": max(ordered),
        "mean": statistics.mean(ordered),
        "total": sum(ordered),
    }


def measurement(step, version):
    return next(item for item in step["measurements"] if item["version"] == version)


def analyze(data):
    summary = {}
    for scenario in data["scenarios"]:
        result = {"eligible_files": scenario["eligible_files"],
                  "selected_files": len(scenario["selected_files"]), "versions": {}}
        for version in ["before", "after"]:
            samples = [measurement(step, version) for step in scenario["updates"]]
            base = next(item["base_id"] for item in scenario["builds"] if item["version"] == version)
            rebuilds = 0
            for item in samples:
                rebuilds += item["base_id"] != base
                base = item["base_id"]
            result["versions"][version] = {
                "refresh_ms": distribution([item["wall_ms"] for item in samples]),
                "rss_bytes": distribution([item["peak_rss_bytes"] for item in samples
                                            if item["peak_rss_bytes"] is not None]),
                "rebuilds": rebuilds,
                "small_merges": sum("compacted" in item["diagnostic"] for item in samples),
                "max_segments": max(item["segments"] for item in samples),
                "final_segments": samples[-1]["segments"],
                "active_mib": samples[-1]["active_bytes"] / MIB,
                "middle_mib": samples[-1]["middle_bytes"] / MIB,
                "retained_mib": samples[-1]["retained_bytes"] / MIB,
            }
        summary[scenario["name"]] = result
    return summary


def tables(data, summary):
    lines = ["| 场景 | 版本 | 首次搜索中位数 ms | p95 ms | 最慢 ms | 100 次合计 s | 基础段重建次数 |",
             "|---|---|---:|---:|---:|---:|---:|"]
    for name, scenario in summary.items():
        for version, values in scenario["versions"].items():
            timing = values["refresh_ms"]
            lines.append(f"| {name} | {version} | {timing['median']:.2f} | {timing['p95']:.2f} | "
                         f"{timing['max']:.2f} | {timing['total']/1000:.2f} | {values['rebuilds']} |")
    lines += ["", "| 场景 | 版本 | 第 100 次活动段数 | 活动索引 MiB | M MiB | 总磁盘 MiB | RSS 中位数 / 最大 MiB |",
              "|---|---|---:|---:|---:|---:|---:|"]
    for name, scenario in summary.items():
        for version, values in scenario["versions"].items():
            rss = values["rss_bytes"]
            lines.append(f"| {name} | {version} | {values['final_segments']} | {values['active_mib']:.2f} | "
                         f"{values['middle_mib']:.2f} | {values['retained_mib']:.2f} | "
                         f"{rss['median']/MIB:.2f} / {rss['max']/MIB:.2f} |")
    lines += ["", "下面按每十次提交汇总首次搜索耗时中位数，包含其中发生的重建或小合并。", "",
              "| 提交区间 | repeated before / after ms | rotating before / after ms |",
              "|---|---:|---:|"]
    for start in range(0, data["commits"], 10):
        cells = []
        for scenario in data["scenarios"]:
            pair = [statistics.median([measurement(step, version)["wall_ms"]
                                       for step in scenario["updates"][start:start+10]])
                    for version in ["before", "after"]]
            cells.append(f"{pair[0]:.2f} / {pair[1]:.2f}")
        lines.append(f"| {start+1}–{min(start+10, data['commits'])} | {' | '.join(cells)} |")
    lines += ["", "稳定查询 AsyncMock、SamplingParams 的两个耗时中位数取算术平均；不包含匹配文件数随提交增长的测试标记。", "",
              "| 场景 | 提交阶段 | no-refresh before / after ms | 默认搜索 before / after ms |",
              "|---|---|---:|---:|"]
    for scenario in data["scenarios"]:
        stages = sorted({(row.get("phase", "updates"), row["step"]) for row in scenario["checkpoints"]},
                        key=lambda stage: (stage[0] != "updates", stage[1]))
        for phase, step in stages:
            cells = []
            for no_refresh in [True, False]:
                pair = [statistics.mean([row["median_ms"] for row in scenario["checkpoints"]
                        if row["step"] == step and row.get("phase", "updates") == phase
                        and row["version"] == version and row["no_refresh"] == no_refresh
                        and row["query"] != "CODERG_COMMIT_BENCH"])
                        for version in ["before", "after"]]
                cells.append(f"{pair[0]:.2f} / {pair[1]:.2f}")
            label = str(step) if phase == "updates" else "100 + full compaction"
            lines.append(f"| {scenario['name']} | {label} | {' | '.join(cells)} |")
    lines += ["", "| 场景 | Git 状态转换 | 变化文件数 | 目标文件完整内容 MiB | before ms | after ms |",
              "|---|---|---:|---:|---:|---:|"]
    for scenario in data["scenarios"]:
        previous = data["commits"]
        for step in scenario["rollbacks"]:
            before, after = [measurement(step, version)["wall_ms"] for version in ["before", "after"]]
            lines.append(f"| {scenario['name']} | {previous} → {step['step']} | {step.get('changed_files', '未记录')} | "
                         f"{step['changed_source_bytes']/MIB:.2f} | {before:.2f} | {after:.2f} |")
            previous = step["step"]
    lines += ["", "| 场景 | 显式命令 | 进程耗时 ms | RSS MiB | 完成后活动段数 |",
              "|---|---|---:|---:|---:|"]
    for scenario in data["scenarios"]:
        for command, sample in zip(["compact", "compact --full"], scenario["compactions"]):
            lines.append(f"| {scenario['name']} | {command} | {sample['wall_ms']:.2f} | "
                         f"{sample['peak_rss_bytes']/MIB:.2f} | {sample['segments']} |")
    return "\n".join(lines) + "\n"


def main():
    data = json.loads((HERE / "results.json").read_text())
    summary = analyze(data)
    (HERE / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    (HERE / "tables.md").write_text(tables(data, summary))
    with (HERE / "updates.csv").open("w", newline="") as output:
        writer = csv.writer(output)
        writer.writerow(["scenario", "step", "version", "refresh_ms", "segments", "rss_bytes", "active_bytes", "retained_bytes"])
        for scenario in data["scenarios"]:
            for step in scenario["updates"]:
                for sample in step["measurements"]:
                    writer.writerow([scenario["name"], step["step"], sample["version"], sample["wall_ms"],
                                     sample["segments"], sample["peak_rss_bytes"], sample["active_bytes"], sample["retained_bytes"]])
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
