#[path = "datapoint/analog_input.rs"]
mod analog_input;
#[path = "datapoint/analog_output.rs"]
mod analog_output;
#[path = "datapoint/binary_input.rs"]
mod binary_input;
#[path = "datapoint/binary_output.rs"]
mod binary_output;
#[path = "datapoint/counter.rs"]
mod counter;
#[path = "datapoint/double_bit_binary_input.rs"]
mod double_bit_binary_input;
#[path = "datapoint/frozen_counter.rs"]
mod frozen_counter;

use analog_input::update_analog_input_value;
use analog_output::update_analog_output_value;
use binary_input::update_binary_input_value;
use binary_output::update_binary_output_value;
use counter::update_counter;
use double_bit_binary_input::update_double_bit_binary_input;
use frozen_counter::update_frozen_counter;

use crate::common_util::generate_random_string;
use dnp3::app::measurement::OctetString;
use dnp3::outstation::database::{Update, UpdateOptions};
use dnp3::outstation::OutstationHandle;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn run_scheduler(outstation: Arc<OutstationHandle>) {
    // Clone the Arc to move it into the closure
    let outstation_clone = Arc::clone(&outstation);

    loop {
        generate_random_update(&outstation_clone);

        // To avoid busy waiting, sleep for a short duration
        thread::sleep(Duration::from_micros(10));
    }
}

pub fn generate_random_update(outstation: &OutstationHandle) {
    outstation.transaction(|db| {
        let value_string = generate_random_string(8);
        // update data
        if let Ok(octet_string) = OctetString::new(value_string.as_bytes()) {
            db.update(0, &octet_string, UpdateOptions::detect_event());
        }

        update_binary_input_value(db);
        update_binary_output_value(db);
        update_analog_input_value(db);
        update_analog_output_value(db);
        update_double_bit_binary_input(db);
        update_counter(db);
        update_frozen_counter(db);
    });
}
