use parking_lot::Mutex;
use std::collections::HashMap;

static METRICS: Mutex<Option<HashMap<String, u64>>> = Mutex::new(None);

pub fn incr(name: &str) {
    let mut g = METRICS.lock();

    let m = g.get_or_insert_with(HashMap::new);

    *m.entry(name.to_string()).or_default() += 1;
}

pub fn snapshot() -> HashMap<String, u64> {
    METRICS.lock().clone().unwrap_or_default()
}
