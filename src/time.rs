// src/time.rs
use dnp3::app::measurement::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> Time {
    let epoch_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    Time::Synchronized(Timestamp::new(epoch_time.as_millis() as u64))
}

// src/outstation.rs
use dnp3::app::attr::{Attribute};
use dnp3::app::{MaybeAsync, Timestamp};
use dnp3::outstation::*;

pub struct ExampleOutstationApplication;
impl OutstationApplication for ExampleOutstationApplication {
    fn support_write_analog_dead_bands(&mut self) -> bool {
        true
    }

    fn write_analog_dead_band(&mut self, index: u16, dead_band: f64) {
        tracing::info!("change analog dead-band {index} to {dead_band}");
    }

    fn write_device_attr(&mut self, attr: Attribute) -> MaybeAsync<bool> {
        tracing::info!("write device attribute: {:?}", attr);
        MaybeAsync::ready(true)
    }
}

pub struct ExampleOutstationInformation;
impl OutstationInformation for ExampleOutstationInformation {}
