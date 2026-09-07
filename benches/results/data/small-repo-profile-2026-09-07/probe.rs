use std::cell::RefCell;
use std::sync::OnceLock;
use std::time::Instant;

static ORIGIN: OnceLock<Instant> = OnceLock::new();
#[derive(serde::Serialize)]
struct Event { name: &'static str, start_ns: u64, duration_ns: u64 }
thread_local! {
    static EVENTS: RefCell<Vec<Event>> = RefCell::new(Vec::with_capacity(64));
    static COUNTS: RefCell<Vec<(&'static str, usize)>> = const { RefCell::new(Vec::new()) };
}
pub struct Span { name: &'static str, start: Instant }
impl Span {
    pub fn new(name: &'static str) -> Self {
        ORIGIN.get_or_init(Instant::now);
        Self { name, start: Instant::now() }
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        let end = Instant::now();
        EVENTS.with(|events| events.borrow_mut().push(Event {
            name: self.name, start_ns: self.start.duration_since(*ORIGIN.get().unwrap()).as_nanos() as u64,
            duration_ns: end.duration_since(self.start).as_nanos() as u64,
        }));
    }
}
pub fn count(name: &'static str, value: usize) {
    COUNTS.with(|counts| counts.borrow_mut().push((name, value)));
}
pub fn emit() {
    EVENTS.with(|events| COUNTS.with(|counts| {
        eprintln!("CODERG_PROFILE {}", serde_json::json!({"events": *events.borrow(), "counts": *counts.borrow()}));
    }));
}
