use dnp3::app::measurement::*;
use std::time::{SystemTime, UNIX_EPOCH};
use dnp3::app::Timestamp;

pub fn get_current_time() -> Time {
    let epoch_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    Time::Synchronized(Timestamp::new(epoch_time.as_millis() as u64))
}
