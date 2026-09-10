use std::{fs::File, io, time::Instant};

pub const SKIP_SYNC: bool = false;

pub fn sync(file: &File, label: &str) -> io::Result<()> {
    let started = Instant::now();
    if !SKIP_SYNC {
        file.sync_all()?;
    }
    eprintln!("PROBE sync_{label} {:.6}", started.elapsed().as_secs_f64() * 1000.0);
    Ok(())
}
