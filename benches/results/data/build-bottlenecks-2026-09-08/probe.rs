use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
pub const TOTAL: usize = 0;
pub const COLLECT_FILES: usize = 1;
pub const GIT: usize = 2;
pub const PIPELINE: usize = 3;
pub const RECEIVE_WAIT: usize = 4;
pub const EXTEND: usize = 5;
pub const FILE_OPEN: usize = 6;
pub const FILE_READ: usize = 7;
pub const GRAM_HASH: usize = 8;
pub const CHUNK_COLLECT: usize = 9;
pub const SEND_WAIT: usize = 10;
pub const SORT: usize = 11;
pub const DEDUP: usize = 12;
pub const SPILL: usize = 13;
pub const SPILL_WRITE: usize = 14;
pub const MERGE: usize = 15;
pub const COUNT_KEYS: usize = 16;
pub const ENCODE: usize = 17;
pub const FINAL_WRITE: usize = 18;
pub const MANIFEST: usize = 19;
pub const CHUNKS: usize = 20;
pub const READ_BYTES: usize = 21;
pub const EMITTED_RECORDS: usize = 22;
pub const SORT_INPUT: usize = 23;
pub const SORT_OUTPUT: usize = 24;
static NS: [AtomicU64; 20] = [const { AtomicU64::new(0) }; 20];
static COUNT: [AtomicU64; 25] = [const { AtomicU64::new(0) }; 25];
pub struct Span(usize, Instant);
impl Span { pub fn new(id: usize) -> Self { Self(id, Instant::now()) } }
impl Drop for Span { fn drop(&mut self) {
    NS[self.0].fetch_add(self.1.elapsed().as_nanos() as u64, Ordering::Relaxed);
    COUNT[self.0].fetch_add(1, Ordering::Relaxed);
} }
pub fn timed<T>(id: usize, f: impl FnOnce() -> T) -> T { let _span = Span::new(id); f() }
pub fn add(id: usize, n: usize) { COUNT[id].fetch_add(n as u64, Ordering::Relaxed); }
pub fn report() {
    let names = ["total", "collect_files", "git", "pipeline", "receive_wait", "extend", "file_open", "file_read", "gram_hash", "chunk_collect", "send_wait", "sort", "dedup", "spill", "spill_write", "merge", "count_keys", "encode", "final_write", "manifest", "chunks", "read_bytes", "emitted_records", "sort_input", "sort_output"];
    let mut values = serde_json::Map::new();
    for (i, name) in names.iter().enumerate() {
        let count = COUNT[i].swap(0, Ordering::Relaxed);
        if i < NS.len() {
            let ns = NS[i].swap(0, Ordering::Relaxed);
            values.insert((*name).to_owned(), serde_json::json!({"ms": ns as f64 / 1e6, "calls": count}));
        } else { values.insert((*name).to_owned(), serde_json::json!(count)); }
    }
    eprintln!("BUILD_PROFILE {}", serde_json::Value::Object(values));
}
