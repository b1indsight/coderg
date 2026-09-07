# Dependency observation

The locked dependency is `ignore 0.4.33`. Its `WalkParallel` creates and joins worker threads on each run (walk.rs:1503–1533).
`Worker::get_work` waits for work or a quit message using the following code (walk.rs:1986–1987):

```rust
let dur = std::time::Duration::from_millis(1);
std::thread::sleep(dur);
```

Source SHA-256: `a81c751dc22763d8efda6fb88fd77c0e03c42fb17538a94497a5737ba8b4b919`.
These source observations and the thread-count experiment suggest scheduler / termination overhead.
The experiment did not separately time those sleeps or count their executions; it does not prove they explain the entire difference.
