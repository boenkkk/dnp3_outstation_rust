use clokwerk::{Scheduler, TimeUnits};
use std::sync::Arc;
use std::thread;
use std::time::{Duration};
use dnp3::app::measurement::{AnalogInput, BinaryInput, Counter, DoubleBitBinaryInput, Flags, FrozenCounter, OctetString};
use dnp3::outstation::database::{Update, UpdateOptions};
use dnp3::outstation::OutstationHandle;

use crate::time::get_current_time;
use crate::util::{generate_random_bool, generate_random_double_bit, generate_random_float, generate_random_int, generate_random_string};

pub fn run_scheduler(outstation: Arc<OutstationHandle>) {
    // Initialize a new scheduler
    let mut scheduler = Scheduler::new();

    // Clone the Arc to move it into the closure
    let outstation_clone = Arc::clone(&outstation);

    // Schedule a task to run every 1 seconds
    scheduler.every(1.seconds()).run(move || {
        println!("Task executed at {:?}", chrono::Utc::now());
        generate_random_update(&outstation_clone);
    });

    // This runs the scheduler in a loop with a sleep to avoid busy waiting
    loop {
        // Run any pending tasks
        scheduler.run_pending();
        // Sleep for half a second
        thread::sleep(Duration::from_millis(500));
    }
}

pub fn generate_random_update(outstation: &OutstationHandle) {
    outstation.transaction(|db| {
        // db.update(generate_random_int(0u32, 2u32) as u16, &OctetString::new("Hai hai".as_bytes()), UpdateOptions::detect_event());
        if let Ok(octet_string) = OctetString::new(generate_random_string(8).as_bytes()) {
            db.update(
                generate_random_int(0u32, 2u32) as u16,
                &octet_string,
                UpdateOptions::detect_event(),
            );
        }

        db.update(generate_random_int(0u32, 2u32) as u16, &BinaryInput::new(generate_random_bool(), Flags::ONLINE, get_current_time()), UpdateOptions::detect_event());
        db.update(generate_random_int(0u32, 2u32) as u16, &AnalogInput::new(generate_random_float(-50f64, 50f64), Flags::ONLINE, get_current_time()), UpdateOptions::detect_event());
        db.update(generate_random_int(0u32, 2u32) as u16, &DoubleBitBinaryInput::new(generate_random_double_bit(), Flags::ONLINE, get_current_time()), UpdateOptions::detect_event());
        db.update(generate_random_int(0u32, 2u32) as u16, &Counter::new(generate_random_int(1u32, 10u32), Flags::ONLINE, get_current_time()), UpdateOptions::detect_event());
        db.update(generate_random_int(0u32, 2u32) as u16, &FrozenCounter::new(generate_random_int(10u32, 20u32), Flags::ONLINE, get_current_time()), UpdateOptions::detect_event());
    });
}
