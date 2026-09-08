"""Classify exclusive call-tree samples; keep waits and overlapping totals separate."""
import argparse
import collections
import json
from pathlib import Path
import re

p = argparse.ArgumentParser()
p.add_argument("samples", nargs="+", help="label=/path/to/sample.txt")
p.add_argument("--output", type=Path, required=True)
a = p.parse_args()
result = {"samples": [], "groups": {}}
wait_re = re.compile(r"__psynch_cvwait|semaphore_wait_trap|__semwait_signal|__ulock_wait|__psynch_mutexwait|swtch_pri|thread_switch|__workq_kernreturn")
io_re = re.compile(r"^(?:read|pread|write|pwrite|close|lstat|stat|fstat|fcntl|fsync|__open|__open_nocancel|__getdirentries64|__rename|__unlinkat|__fstat64|__lstat64)\s+\(in ")

for spec in a.samples:
    label, filename = spec.split("=", 1)
    text = Path(filename).read_text()
    graph = text.split("Call graph:\n", 1)[1].split("Total number in stack", 1)[0]
    nodes = []
    stack = []
    extraction_roots = set()
    for line in graph.splitlines():
        m = re.match(r"^([ +!:|]*)(\d+) (.*)$", line)
        if not m:
            continue
        depth, count, name = len(m[1]), int(m[2]), m[3]
        while stack and nodes[stack[-1]]["depth"] >= depth:
            stack.pop()
        parent = stack[-1] if stack else None
        index = len(nodes)
        root = nodes[parent]["root"] if parent is not None else index
        node = {"depth": depth, "count": count, "self": count, "name": name, "parent": parent, "root": root}
        nodes.append(node)
        if parent is not None:
            nodes[parent]["self"] -= count
        if "coderg::ngram::hashes_for_" in name:
            extraction_roots.add(root)
        stack.append(index)
    counts = collections.Counter()
    for i, node in enumerate(nodes):
        own = node["self"]
        assert own >= 0, node
        if not own:
            continue
        names = []
        while i is not None:
            names.append(nodes[i]["name"])
            i = nodes[i]["parent"]
        gram = any("coderg::ngram::hashes_for_" in n for n in names)
        extraction_thread = node["root"] in extraction_roots
        insert = any("hashbrown::map::HashMap" in n and "::insert::" in n for n in names)
        hashbrown = any("hashbrown::" in n for n in names)
        if wait_re.search(node["name"]):
            category = "wait"
        elif extraction_thread and any("SpecFromIterNested" in n and "::from_iter::" in n for n in names):
            category = "separate_set_to_vec"
        elif extraction_thread and insert:
            # Optimized stacks can omit the outer extraction function at some
            # call sites. Keep these insert samples with their producer thread.
            category = "hashset_insert"
        elif extraction_thread and hashbrown:
            category = "hashset_other"
        elif gram:
            if insert:
                category = "hashset_insert"
            elif hashbrown:
                category = "hashset_other"
            else:
                category = "gram_scan_hash_and_inline_output"
        elif io_re.search(node["name"]):
            category = "file_syscalls"
        elif any("rayon::slice::sort::" in n for n in names):
            category = "sort"
        elif any("PostingsBuilder::write::" in n or "coderg::segment::" in n for n in names):
            category = "finalize_postings"
        elif any("PostingsBuilder::extend::" in n for n in names):
            category = "collect_records"
        elif any("coderg::index::collect_files::" in n or "ignore::" in n or "git2::" in n or "coderg::git_state::" in n for n in names):
            category = "metadata"
        else:
            category = "other"
        counts[category] += own
    assert sum(counts.values()) == sum(n["count"] for n in nodes if n["parent"] is None)
    result["samples"].append({"label": label, "path": filename, "counts": dict(counts)})
    totals = result["groups"].setdefault(label, {})
    for key, value in counts.items():
        totals[key] = totals.get(key, 0) + value
for label, counts in list(result["groups"].items()):
    extraction_keys = ("hashset_insert", "hashset_other", "gram_scan_hash_and_inline_output", "separate_set_to_vec")
    extraction = sum(counts.get(k, 0) for k in extraction_keys)
    nonwait = sum(v for k, v in counts.items() if k != "wait")
    result["groups"][label] = {
        "counts": counts,
        "extraction_samples": extraction,
        "nonwait_samples": nonwait,
        "within_extraction_percent": {k: 100 * counts.get(k, 0) / extraction for k in extraction_keys},
        "within_nonwait_percent": {k: 100 * v / nonwait for k, v in counts.items() if k != "wait"},
    }
a.output.write_text(json.dumps(result, indent=2) + "\n")
for label, row in result["groups"].items():
    print(label, "extraction", {k: round(v, 2) for k, v in row["within_extraction_percent"].items()})
    print(label, "nonwait", {k: round(v, 2) for k, v in row["within_nonwait_percent"].items()})
