use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_millis()
}
