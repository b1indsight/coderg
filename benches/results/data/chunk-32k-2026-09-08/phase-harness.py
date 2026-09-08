"""Fresh builds, phase measurements, byte parity, and optional macOS stacks."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import statistics
import subprocess
import tempfile
import time

p = argparse.ArgumentParser()
p.add_argument("--control", type=Path, required=True)
p.add_argument("--probe", type=Path, required=True)
p.add_argument("--corpus", action="append", required=True)
p.add_argument("--output", type=Path, required=True)
p.add_argument("--iterations", type=int, default=7)
p.add_argument("--warmup", type=int, default=2)
p.add_argument("--sample", action="store_true")
a = p.parse_args()
a.output.mkdir(parents=True, exist_ok=False)
env = os.environ.copy()
env.pop("RAYON_NUM_THREADS", None)
env.pop("CODERG_PROFILE_REPEAT", None)
randomizer = random.Random(20260908)

def digest(path):
    with path.open("rb") as f:
        return hashlib.file_digest(f, "sha256").hexdigest()

result = {"versions": {n: {"path": str(b), "sha256": digest(b)} for n, b in (("control", a.control), ("probe", a.probe))}, "runs": [], "samples": []}
references = {}
for corpus in a.corpus:
    name, source = corpus.split("=", 1)
    if a.sample:
        # Keep one process alive long enough to capture multiple full builds.
        sample_env = env | {"CODERG_PROFILE_REPEAT": "120" if name == "few_large_64" else "12"}
        with tempfile.TemporaryDirectory(prefix="index-", dir=a.output) as index:
            log = a.output / f"{name}.builds.stderr.txt"
            with log.open("w") as stderr:
                child = subprocess.Popen([str(a.probe), "index", source, "--index-dir", index, "--build-memory-mib", "256"], env=sample_env, stdout=subprocess.DEVNULL, stderr=stderr)
                time.sleep(0.1)
                sampled = subprocess.run(["/usr/bin/sample", str(child.pid), "5", "1", "-file", str(a.output / f"{name}.sample.txt")], capture_output=True, text=True)
                if sampled.returncode != 0:
                    child.terminate()
                child.wait()
            result["samples"].append({"corpus": name, "pid": child.pid, "build_returncode": child.returncode, "sample_returncode": sampled.returncode, "sample_stdout": sampled.stdout, "sample_stderr": sampled.stderr})
            (a.output / "results.json").write_text(json.dumps(result, indent=2) + "\n")
            assert sampled.returncode == 0, sampled.stderr
        print(name, "sample complete", flush=True)
        continue
    for iteration in range(-a.warmup, a.iterations):
        versions = [("control", a.control), ("probe", a.probe)]
        randomizer.shuffle(versions)
        for version, binary in versions:
            with tempfile.TemporaryDirectory(prefix="index-", dir=a.output) as index_str:
                index = Path(index_str)
                command = [str(binary), "index", source, "--index-dir", str(index), "--build-memory-mib", "256"]
                usage_before = resource.getrusage(resource.RUSAGE_CHILDREN)
                started = time.perf_counter()
                process = subprocess.run(command, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
                elapsed_ms = (time.perf_counter() - started) * 1000
                usage_after = resource.getrusage(resource.RUSAGE_CHILDREN)
                assert process.returncode == 0, process.stderr
                stem = f"{name}-{iteration}-{version}"
                (a.output / f"{stem}.stderr.txt").write_text(process.stderr)
                manifest = json.loads((index / "manifest.json").read_text())
                segment = manifest["segments"][0]
                fingerprint = {key: {"bytes": (index / segment[key]).stat().st_size, "sha256": digest(index / segment[key])} for key in ("lookup", "postings")}
                docs = [(doc["path"], doc["len"], doc["searchable"], doc["active"]) for doc in manifest["documents"]]
                fingerprint["documents"] = hashlib.sha256(json.dumps(docs).encode()).hexdigest()
                if name not in references:
                    references[name] = fingerprint
                assert fingerprint == references[name], stem
                assert not list(index.glob(".build-*"))
                phase = next((json.loads(line.removeprefix("BUILD_PROFILE ")) for line in process.stderr.splitlines() if line.startswith("BUILD_PROFILE ")), None)
                row = {"corpus": name, "source": source, "version": version, "iteration": iteration, "warmup": iteration < 0, "command": command, "wall_ms": elapsed_ms, "user_cpu_ms": (usage_after.ru_utime - usage_before.ru_utime) * 1000, "system_cpu_ms": (usage_after.ru_stime - usage_before.ru_stime) * 1000, "profile": phase, "fingerprint": fingerprint, "files": sum(doc[2] for doc in docs), "source_bytes": sum(doc[1] for doc in docs if doc[2])}
                result["runs"].append(row)
        (a.output / "results.json").write_text(json.dumps(result, indent=2) + "\n")
    rows = [r for r in result["runs"] if r["corpus"] == name and not r["warmup"]]
    print(name, {v: round(statistics.median(r["wall_ms"] for r in rows if r["version"] == v), 3) for v in ("control", "probe")}, flush=True)
(a.output / "results.json").write_text(json.dumps(result, indent=2) + "\n")
