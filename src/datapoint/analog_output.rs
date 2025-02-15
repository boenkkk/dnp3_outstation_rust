use crate::common_util::{generate_random_float, generate_random_int};
use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{AnalogOutputStatus, Flags};
use dnp3::outstation::database::{
    Add, AnalogOutputStatusConfig, Database, EventClass, Update, UpdateOptions,
};
use std::env;
use std::ops::Index;
use std::time::Duration;
use tokio::time::Instant;

pub fn initial_analog_output(db: &mut Database) {
    let dnp3_analog_output_total = env::var("DNP3_ANALOG_OUTPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    if dnp3_analog_output_total > 0 {
        let dnp3_analog_output_init_value: Vec<f64> =
            serde_json::from_str(env::var("DNP3_ANALOG_OUTPUT_INIT_VALUE").unwrap().as_str())
                .expect("Failed to parse DNP3_ANALOG_INPUT_INIT_VALUE");
        for i in 0..dnp3_analog_output_total {
            let i_usize: usize = i as usize;

            db.add(
                i,
                Some(EventClass::Class1),
                AnalogOutputStatusConfig::default(),
            );

            db.update(
                i,
                &AnalogOutputStatus::new(
                    *dnp3_analog_output_init_value.index(i_usize),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

pub fn update_analog_output_value(db: &mut Database) {
    let dnp3_analog_output_random_update_interval =
        env::var("DNP3_ANALOG_OUTPUT_RANDOM_UPDATE_INTERVAL")
            .unwrap()
            .parse::<u64>()
            .unwrap_or(0);

    if dnp3_analog_output_random_update_interval > 0 {
        static mut LAST_RUN: Option<Instant> = None;

        // Safely update and access LAST_RUN
        let now = Instant::now();
        let should_run = unsafe {
            if let Some(last_run) = LAST_RUN {
                if now.duration_since(last_run)
                    >= Duration::from_millis(dnp3_analog_output_random_update_interval)
                {
                    LAST_RUN = Some(now);
                    true
                } else {
                    false
                }
            } else {
                LAST_RUN = Some(now);
                true
            }
        };

        if should_run {
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

                    let dnp3_analog_output_value: Vec<Vec<f64>> = serde_json::from_str(
                        env::var("DNP3_ANALOG_OUTPUT_RANGE").unwrap().as_str(),
                    )
                    .expect("Failed to parse DNP3_ANALOG_OUTPUT_RANGE");

                    let dnp3_analog_output_low =
                        dnp3_analog_output_value[analog_output_index as usize][0];
                    let dnp3_analog_output_high =
                        dnp3_analog_output_value[analog_output_index as usize][1];

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
    }
}
