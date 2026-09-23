//! One process contract for verification, latency, and separate RSS sampling.
use anyhow::{Context, Result, bail, ensure};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Read,
    path::Path,
    process::{Command, Output, Stdio},
    time::Instant,
};

pub fn checked(command: &mut Command, search: bool) -> Result<Output> {
    let output = command
        .output()
        .with_context(|| format!("launch {command:?}"))?;
    ensure!(
        output.status.success() || (search && output.status.code() == Some(1)),
        "{command:?}: {}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(output)
}

pub fn text(command: &mut Command) -> Result<String> {
    Ok(String::from_utf8(checked(command, false)?.stdout)?
        .trim()
        .into())
}

pub fn git(root: &Path, args: &[&str]) -> Result<String> {
    text(Command::new("git").arg("-C").arg(root).args(args))
}

pub fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

/// Preserve duplicate lines and arbitrary bytes, normalizing only a leading ./.
pub fn normalized(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut lines: Vec<_> = bytes
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|line| line.strip_prefix(b"./").unwrap_or(line).to_vec())
        .collect();
    lines.sort();
    lines
}

/// Time a fresh CLI process, with output discarded exactly as in all other cases.
pub fn timed(command: &mut Command, search: bool) -> Result<(f64, Output)> {
    command.stdout(Stdio::null()).stderr(Stdio::piped());
    let start = Instant::now();
    let output = checked(command, search)?;
    Ok((start.elapsed().as_secs_f64() * 1000.0, output))
}

/// RSS runs are independent from latency runs; platform units become bytes.
pub fn rss(command: &Command, search: bool) -> Result<Option<u64>> {
    if !cfg!(any(target_os = "macos", target_os = "linux")) {
        return Ok(None);
    }
    let mut wrapper = Command::new("/usr/bin/time");
    if cfg!(target_os = "macos") {
        wrapper.arg("-l");
    } else {
        wrapper.args(["-f", "CODERG_RSS_KIB=%M"]);
    }
    wrapper.arg(command.get_program()).args(command.get_args());
    if let Some(dir) = command.get_current_dir() {
        wrapper.current_dir(dir);
    }
    for (key, val) in command.get_envs() {
        match val {
            Some(v) => {
                wrapper.env(key, v);
            }
            None => {
                wrapper.env_remove(key);
            }
        }
    }
    wrapper.stdout(Stdio::null());
    let output = checked(&mut wrapper, search)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    parse_rss(&stderr, cfg!(target_os = "macos")).map(Some)
}

pub fn parse_rss(stderr: &str, macos: bool) -> Result<u64> {
    for line in stderr.lines() {
        if macos && line.contains("maximum resident set size") {
            return Ok(line
                .split_whitespace()
                .next()
                .context("empty RSS")?
                .parse()?);
        }
        if !macos && let Some(value) = line.strip_prefix("CODERG_RSS_KIB=") {
            return value
                .trim()
                .parse::<u64>()?
                .checked_mul(1024)
                .context("RSS overflow");
        }
    }
    bail!("missing RSS measurement: {stderr}")
}

/// Seeded Fisher-Yates order, persisted with each sample by the caller.
pub fn shuffle<T>(values: &mut [T], state: &mut u64) {
    for i in (1..values.len()).rev() {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        values.swap(i, (*state >> 32) as usize % (i + 1));
    }
}

pub fn machine_metadata() -> serde_json::Value {
    #[cfg(target_os = "macos")]
    {
        match Command::new("/usr/sbin/sysctl")
            .args([
                "machdep.cpu.brand_string",
                "hw.memsize",
                "hw.logicalcpu",
                "kern.osproductversion",
            ])
            .output()
        {
            Ok(output) if output.status.success() => {
                serde_json::json!({"sysctl": String::from_utf8_lossy(&output.stdout)})
            }
            Ok(output) => {
                serde_json::json!({"unavailable": String::from_utf8_lossy(&output.stderr)})
            }
            Err(error) => serde_json::json!({"unavailable": error.to_string()}),
        }
    }
    #[cfg(target_os = "linux")]
    {
        serde_json::json!({"cpuinfo": std::fs::read_to_string("/proc/cpuinfo").ok(), "meminfo": std::fs::read_to_string("/proc/meminfo").ok()})
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        serde_json::json!({"unavailable": "no machine metadata backend for this platform"})
    }
}
