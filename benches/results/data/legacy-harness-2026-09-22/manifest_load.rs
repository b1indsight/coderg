use anyhow::{Result, bail};
use clap::Parser;
use serde::Serialize;
use std::{fs, hint::black_box, path::PathBuf, time::Instant};

#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest;

#[derive(Parser)]
#[command(about = "Compare JSON and binary manifest loading using the same records")]
struct Args {
    /// Existing manifest.bin or legacy manifest.json.
    #[arg(long)]
    path: PathBuf,
    #[arg(long, default_value_t = 100)]
    iterations: usize,
    #[arg(long, default_value_t = 10)]
    warmup: usize,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long, hide = true)]
    bench: bool,
}

#[derive(Default, Serialize)]
struct Samples {
    read_ms: Vec<f64>,
    decode_ms: Vec<f64>,
    total_ms: Vec<f64>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.iterations == 0 {
        bail!("--iterations must be positive");
    }
    let source = manifest::read(&args.path)?;
    let json = serde_json::to_vec_pretty(&source)?;
    let binary = manifest::encode(&source)?;
    assert_eq!(manifest::decode(&json)?, source);
    assert_eq!(manifest::decode(&binary)?, source);
    let temp = tempfile::tempdir()?;
    let paths = [
        temp.path().join("manifest.json"),
        temp.path().join("manifest.bin"),
    ];
    fs::write(&paths[0], &json)?;
    fs::write(&paths[1], &binary)?;
    let mut samples = [Samples::default(), Samples::default()];
    // Alternate the starting format each round, with identical record contents.
    for round in 0..args.warmup + args.iterations {
        for mode in [round % 2, 1 - round % 2] {
            let started = Instant::now();
            let bytes = fs::read(&paths[mode])?;
            let read_ms = started.elapsed().as_secs_f64() * 1000.0;
            let decoding = Instant::now();
            let decoded = black_box(manifest::decode(black_box(&bytes))?);
            let decode_ms = decoding.elapsed().as_secs_f64() * 1000.0;
            let total_ms = started.elapsed().as_secs_f64() * 1000.0;
            // Destruction is outside the load timer in both modes.
            drop(decoded);
            if round >= args.warmup {
                samples[mode].read_ms.push(read_ms);
                samples[mode].decode_ms.push(decode_ms);
                samples[mode].total_ms.push(total_ms);
            }
        }
    }
    let summary = |sample: &Samples| {
        serde_json::json!({
            "read_ms": median(&sample.read_ms), "decode_ms": median(&sample.decode_ms),
            "total_ms": median(&sample.total_ms),
        })
    };
    let report = serde_json::json!({
        "path": args.path, "iterations": args.iterations, "warmup": args.warmup,
        "documents": source.documents.len(), "files": source.source_state.len(),
        "json_bytes": json.len(), "binary_bytes": binary.len(),
        "median_json": summary(&samples[0]), "median_binary": summary(&samples[1]),
        "samples_json": samples[0], "samples_binary": samples[1],
    });
    if let Some(path) = args.output {
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        serde_json::to_writer_pretty(file, &report)?;
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "json_bytes": json.len(), "binary_bytes": binary.len(),
            "json": summary(&samples[0]), "binary": summary(&samples[1]),
        }))?
    );
    Ok(())
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    (sorted[(sorted.len() - 1) / 2] + sorted[sorted.len() / 2]) / 2.0
}
