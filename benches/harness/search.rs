//! Default build, search, and manifest load measurements.
use super::{
    process,
    session::{Fixture, Session},
};
use anyhow::{Result, ensure};
use std::{hint::black_box, time::Instant};

pub fn build(s: &mut Session<'_>) -> Result<()> {
    let revision = s.corpus.commits.last().unwrap().clone();
    let case = "default";
    eprintln!("{} build {case}", s.corpus.config.name);
    for round in 0..1 + s.config.rounds + s.config.rss_runs {
        let f = s.fixture(&revision)?;
        for v in 0..s.variants.len() {
            let name = s.variants[v].name.clone();
            let mut command = s.build(&f, v);
            if round == 0 {
                process::checked(&mut command, false)?;
            } else if round <= s.config.rounds {
                s.measure("build", case, &name, round - 1, &mut command, false)?;
            } else {
                s.resource(
                    "build",
                    case,
                    &name,
                    round - s.config.rounds - 1,
                    &command,
                    false,
                )?;
            }
        }
        if round > 0 && round <= s.config.rounds {
            s.verify(&f, case, &s.corpus.config.queries.clone())?;
            s.state(&f, "build", case, round - 1)?;
        }
    }
    Ok(())
}

pub fn search(s: &mut Session<'_>) -> Result<()> {
    let f = s.fixture(s.corpus.commits.last().unwrap())?;
    s.initialize(&f)?;
    checkpoint(s, &f, "initial", 0)?;
    Ok(())
}

/// Interleave engines and queries per sweep. All engines use identical output sinks.
pub fn checkpoint(s: &mut Session<'_>, f: &Fixture, state: &str, round: usize) -> Result<()> {
    let queries = s.corpus.config.queries.clone();
    s.verify(f, state, &queries)?;
    let before: Vec<_> = f
        .indexes
        .iter()
        .map(|p| process::hash_file(&p.join("manifest.bin")))
        .collect::<Result<_>>()?;
    let mut jobs = Vec::new();
    for q in 0..queries.len() {
        for v in 0..s.variants.len() {
            jobs.push((q, Some(v)));
        }
        jobs.push((q, None));
    }
    for sweep in 0..s.config.warmup + s.config.samples + s.config.rss_runs {
        process::shuffle(&mut jobs, &mut s.seed);
        for &(q, v) in &jobs {
            let (name, mut command) = match v {
                Some(v) => (s.variants[v].name.clone(), s.search(f, v, &queries[q])),
                None => ("rg".into(), s.rg(f, &queries[q])),
            };
            let case = format!("{state}-{}", queries[q].name);
            if sweep < s.config.warmup {
                process::timed(&mut command, true)?;
            } else if sweep < s.config.warmup + s.config.samples {
                s.measure(
                    "search",
                    &case,
                    &name,
                    round * s.config.samples + sweep - s.config.warmup,
                    &mut command,
                    true,
                )?;
            } else {
                s.resource(
                    "search",
                    &case,
                    &name,
                    sweep - s.config.warmup - s.config.samples,
                    &command,
                    true,
                )?;
            }
        }
    }
    for (i, index) in f.indexes.iter().enumerate() {
        ensure!(
            process::hash_file(&index.join("manifest.bin"))? == before[i],
            "read-only checkpoint changed manifest"
        );
    }
    Ok(())
}

/// Measure the production manifest search loader on the actual index artifact.
/// Segment loading, freshness checks, validation and destruction are outside timing.
pub fn manifest(s: &mut Session<'_>) -> Result<()> {
    let f = s.fixture(s.corpus.commits.last().unwrap())?;
    s.initialize(&f)?;
    for v in 0..s.variants.len() {
        let path = f.indexes[v].join(crate::manifest::FILE_NAME);
        let before = process::hash_file(&path)?;
        // Validate the actual artifact outside timing. View::open returns the
        // checkout's native search representation (main also returns a header).
        crate::manifest::read(&path)?;
        for round in 0..s.config.warmup + s.config.samples {
            let start = Instant::now();
            let (case, elapsed) = match crate::manifest::view::View::open(black_box(&path))? {
                Some(view) => {
                    black_box(&view);
                    let elapsed = start.elapsed();
                    drop(view);
                    ("search_view", elapsed)
                }
                None => {
                    // Match load_for_search's legacy-format fallback, including
                    // the unsuccessful view attempt in the measured duration.
                    let manifest = crate::manifest::read(black_box(&path))?;
                    black_box(&manifest);
                    let elapsed = start.elapsed();
                    drop(manifest);
                    ("search_full_fallback", elapsed)
                }
            };
            if round >= s.config.warmup {
                // The manifest loader runs in-process using current production code.
                let variant = "current";
                s.record(
                    "manifest",
                    case,
                    variant,
                    round - s.config.warmup,
                    "load",
                    elapsed.as_secs_f64() * 1000.0,
                    "ms",
                )?;
            }
        }
        ensure!(
            process::hash_file(&path)? == before,
            "manifest loading changed manifest"
        );
    }
    Ok(())
}
