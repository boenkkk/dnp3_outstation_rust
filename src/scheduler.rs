use crate::common_util::{
    generate_random_bool, generate_random_double_bit, generate_random_float, generate_random_int,
    generate_random_string,
};
use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{
    AnalogInput, AnalogOutputStatus, BinaryInput, BinaryOutputStatus, Counter,
    DoubleBitBinaryInput, Flags, FrozenCounter, OctetString,
};
use dnp3::outstation::database::{Database, Update, UpdateOptions};
use dnp3::outstation::OutstationHandle;
use std::sync::Arc;
use std::time::Duration;
use std::{env, thread};
use tokio::time::Instant;

pub fn run_scheduler(outstation: Arc<OutstationHandle>) {
    // Clone the Arc to move it into the closure
    let outstation_clone = Arc::clone(&outstation);

    // This runs the scheduler in a loop with precise timing
    let mut last_run = Instant::now();

    let dnp3_scheduler_interval = env::var("DNP3_SCHEDULER_INTERVAL")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(0);
    if dnp3_scheduler_interval > 0 {
        loop {
            let now = Instant::now();
            if now.duration_since(last_run) >= Duration::from_millis(dnp3_scheduler_interval) {
                last_run = now;
                generate_random_update(&outstation_clone);
            }
            // To avoid busy waiting, sleep for a short duration
            thread::sleep(Duration::from_micros(dnp3_scheduler_interval / 2));
        }
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

fn update_binary_input_value(db: &mut Database) {
    let dnp3_binary_input_total = env::var("DNP3_BINARY_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(0);

    if dnp3_binary_input_total > 0 {
        let is_random_update = env::var("DNP3_BINARY_INPUT_RANDOM_UPDATE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Failed to parse DNP3_BINARY_INPUT_RANDOM_UPDATE as a boolean");

        if is_random_update {
            let binary_input_index =
                generate_random_int(0u32, dnp3_binary_input_total as u32 - 1) as u16;
            db.update(
                binary_input_index,
                &BinaryInput::new(generate_random_bool(), Flags::ONLINE, get_current_time()),
                UpdateOptions::detect_event(),
            );
        }
    }
}

fn update_binary_output_value(db: &mut Database) {
    let dnp3_binary_output_total = env::var("DNP3_BINARY_OUTPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(0);

    if dnp3_binary_output_total > 0 {
        let is_random_update = env::var("DNP3_BINARY_OUTPUT_RANDOM_UPDATE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Failed to parse DNP3_BINARY_OUTPUT_RANDOM_UPDATE as a boolean");

        if is_random_update {
            let binary_output_index =
                generate_random_int(0u32, dnp3_binary_output_total as u32 - 1) as u16;
            db.update(
                binary_output_index,
                &BinaryOutputStatus::new(generate_random_bool(), Flags::ONLINE, get_current_time()),
                UpdateOptions::detect_event(),
            );
        }
    }
}

fn update_analog_input_value(db: &mut Database) {
    let dnp3_analog_input_total = env::var("DNP3_ANALOG_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(0);

    if dnp3_analog_input_total > 0 {
        let is_random_update = env::var("DNP3_ANALOG_INPUT_RANDOM_UPDATE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Failed to parse DNP3_ANALOG_INPUT_RANDOM_UPDATE as a boolean");

        if is_random_update {
            let analog_input_index =
                generate_random_int(0u32, dnp3_analog_input_total as u32 - 1) as u16;

            let dnp3_analog_input_value: Vec<Vec<f64>> =
                serde_json::from_str(env::var("DNP3_ANALOG_INPUT_RANGE").unwrap().as_str())
                    .expect("Failed to parse DNP3_ANALOG_INPUT_RANGE");

            let dnp3_analog_input_low = dnp3_analog_input_value[analog_input_index as usize][0];
            let dnp3_analog_input_high = dnp3_analog_input_value[analog_input_index as usize][1];

            db.update(
                analog_input_index,
                &AnalogInput::new(
                    generate_random_float(
                        dnp3_analog_input_low.into(),
                        dnp3_analog_input_high.into(),
                    ),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

fn update_analog_output_value(db: &mut Database) {
    let dnp3_analog_output_total = env::var("DNP3_ANALOG_OUTPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(0);

    if dnp3_analog_output_total > 0 {
        let is_random_update = env::var("DNP3_ANALOG_OUTPUT_RANDOM_UPDATE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Failed to parse DNP3_ANALOG_OUTPUT_RANDOM_UPDATE as a boolean");

        if is_random_update {
            let analog_output_index =
                generate_random_int(0u32, dnp3_analog_output_total as u32 - 1) as u16;

            let dnp3_analog_output_value: Vec<Vec<f64>> =
                serde_json::from_str(env::var("DNP3_ANALOG_OUTPUT_RANGE").unwrap().as_str())
                    .expect("Failed to parse DNP3_ANALOG_OUTPUT_RANGE");

            let dnp3_analog_output_low = dnp3_analog_output_value[analog_output_index as usize][0];
            let dnp3_analog_output_high = dnp3_analog_output_value[analog_output_index as usize][1];

            db.update(
                analog_output_index,
                &AnalogOutputStatus::new(
                    generate_random_float(
                        dnp3_analog_output_low.into(),
                        dnp3_analog_output_high.into(),
                    ),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

fn update_double_bit_binary_input(db: &mut Database) {
    let dnp3_double_bit_binary_input_total = env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(0);

    if dnp3_double_bit_binary_input_total > 0 {
        let is_random_update = env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_RANDOM_UPDATE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Failed to parse DNP3_DOUBLE_BIT_BINARY_INPUT_RANDOM_UPDATE as a boolean");

        if is_random_update {
            let double_bit_binary_input_index =
                generate_random_int(0u32, dnp3_double_bit_binary_input_total as u32 - 1) as u16;

            db.update(
                double_bit_binary_input_index,
                &DoubleBitBinaryInput::new(
                    generate_random_double_bit(),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

fn update_counter(db: &mut Database) {
    let dnp3_counter_total = env::var("DNP3_COUNTER_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(0);

    if dnp3_counter_total > 0 {
        let is_random_update = env::var("DNP3_COUNTER_RANDOM_UPDATE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Failed to parse DNP3_COUNTER_RANDOM_UPDATE as a boolean");

        if is_random_update {
            let counter_index = generate_random_int(0u32, dnp3_counter_total as u32 - 1) as u16;

            let dnp3_counter_value: Vec<Vec<u32>> =
                serde_json::from_str(env::var("DNP3_COUNTER_RANGE").unwrap().as_str())
                    .expect("Failed to parse DNP3_COUNTER_RANGE");

            let dnp3_counter_low = dnp3_counter_value[counter_index as usize][0];
            let dnp3_counter_high = dnp3_counter_value[counter_index as usize][1];

            db.update(
                counter_index,
                &Counter::new(
                    generate_random_int(dnp3_counter_low, dnp3_counter_high),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

fn update_frozen_counter(db: &mut Database) {
    let dnp3_frozen_counter_total = env::var("DNP3_FROZEN_COUNTER_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(0);

    if dnp3_frozen_counter_total > 0 {
        let is_random_update = env::var("DNP3_FROZEN_COUNTER_RANDOM_UPDATE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Failed to parse DNP3_FROZEN_COUNTER_RANDOM_UPDATE as a boolean");

        if is_random_update {
            let frozen_counter_index =
                generate_random_int(0u32, dnp3_frozen_counter_total as u32 - 1) as u16;

            let dnp3_frozen_counter_value: Vec<Vec<u32>> =
                serde_json::from_str(env::var("DNP3_FROZEN_COUNTER_RANGE").unwrap().as_str())
                    .expect("Failed to parse DNP3_FROZEN_COUNTER_RANGE");

            let dnp3_frozen_counter_low =
                dnp3_frozen_counter_value[frozen_counter_index as usize][0];
            let dnp3_frozen_counter_high =
                dnp3_frozen_counter_value[frozen_counter_index as usize][1];

            db.update(
                frozen_counter_index,
                &FrozenCounter::new(
                    generate_random_int(dnp3_frozen_counter_low, dnp3_frozen_counter_high),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}
