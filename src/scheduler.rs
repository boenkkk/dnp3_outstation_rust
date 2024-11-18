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
        // declare variables
        let flag = Flags::ONLINE;
        let current_time = get_current_time();
        let index = generate_random_int(0u32, 2u32) as u16;
        let value_string = generate_random_string(8);
        let value_float = generate_random_float(-50f64, 50f64);
        let value_boolean = generate_random_bool();
        let value_double_bit = generate_random_double_bit();
        let value_int = generate_random_int(1u32, 10u32);
        let detect_event = UpdateOptions::detect_event();

        // update data
        if let Ok(octet_string) = OctetString::new(value_string.as_bytes()) {
            db.update(index, &octet_string, detect_event);
        }
        db.update(index, &AnalogInput::new(value_float, flag, current_time), detect_event);
        db.update(index, &BinaryInput::new(value_boolean, flag, current_time), detect_event);
        db.update(index, &DoubleBitBinaryInput::new(value_double_bit, flag, current_time), detect_event);
        db.update(index, &Counter::new(value_int, flag, current_time), detect_event);
        db.update(index, &FrozenCounter::new(value_int, flag, current_time), detect_event);
    });
}
