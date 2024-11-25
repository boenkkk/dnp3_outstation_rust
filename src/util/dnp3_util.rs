use dnp3::app::measurement::Time;
use dnp3::app::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> Time {
    let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    Time::Synchronized(Timestamp::new(epoch_time.as_millis() as u64))
}
