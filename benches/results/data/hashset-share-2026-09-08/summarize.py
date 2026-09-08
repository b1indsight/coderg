"""Count nonoverlapping insert subtrees within gram extraction call stacks."""
import argparse
import json
from pathlib import Path
import re

p = argparse.ArgumentParser()
p.add_argument("samples", nargs="+", type=Path)
p.add_argument("--output", type=Path, required=True)
a = p.parse_args()
result = {"samples": []}
for path in a.samples:
    text = path.read_text()
    graph = text.split("Call graph:\n", 1)[1].split("Total number in stack", 1)[0]
    stack = []
    counts = {"gram": 0, "insert": 0, "hashbrown": 0}
    for line in graph.splitlines():
        match = re.match(r"^([ +!:|]*)(\d+) (.*)$", line)
        if not match:
            continue
        depth, count, name = len(match[1]), int(match[2]), match[3]
        while stack and stack[-1]["depth"] >= depth:
            stack.pop()
        gram = "coderg::ngram::hashes_for_chunk::" in name
        insert = "hashbrown::map::HashMap" in name and "::insert::" in name
        hashbrown = "hashbrown::" in name
        inside_gram = any(n["gram"] for n in stack)
        if gram and not inside_gram:
            counts["gram"] += count
        if inside_gram and insert and not any(n["insert"] for n in stack):
            counts["insert"] += count
        if inside_gram and hashbrown and not any(n["hashbrown"] for n in stack):
            counts["hashbrown"] += count
        stack.append({"depth": depth, "gram": gram, "insert": insert, "hashbrown": hashbrown})
    assert 0 < counts["insert"] <= counts["hashbrown"] <= counts["gram"], counts
    top = text.split("Sort by top of stack, same collapsed (when >= 5):\n", 1)[1].split("\nBinary Images:", 1)[0]
    row = {"path": str(path), **counts, "insert_share_percent": 100 * counts["insert"] / counts["gram"], "hashbrown_share_percent": 100 * counts["hashbrown"] / counts["gram"], "top_of_stack": top}
    result["samples"].append(row)
totals = {key: sum(row[key] for row in result["samples"]) for key in ("gram", "insert", "hashbrown")}
result["total"] = {**totals, "insert_share_percent": 100 * totals["insert"] / totals["gram"], "hashbrown_share_percent": 100 * totals["hashbrown"] / totals["gram"]}
a.output.write_text(json.dumps(result, indent=2) + "\n")
for row in result["samples"]:
    print({k: v for k, v in row.items() if k != "top_of_stack"})
print("total", result["total"])
