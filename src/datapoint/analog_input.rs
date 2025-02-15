use crate::common_util::generate_random_float;
use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{AnalogInput, Flags};
use dnp3::outstation::database::{
    Add, AnalogInputConfig, Database, EventAnalogInputVariation, EventClass,
    StaticAnalogInputVariation, Update, UpdateOptions,
};
use std::env;
use std::ops::Index;
use std::time::Duration;
use tokio::time::Instant;

pub fn initial_analog_input(db: &mut Database) {
    let dnp3_analog_input_total = env::var("DNP3_ANALOG_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    if dnp3_analog_input_total > 0 {
        let dnp3_analog_input_init_value: Vec<f64> =
            serde_json::from_str(env::var("DNP3_ANALOG_INPUT_INIT_VALUE").unwrap().as_str())
                .expect("Failed to parse DNP3_ANALOG_INPUT_INIT_VALUE");
        for i in 0..dnp3_analog_input_total {
            let i_usize: usize = i as usize;

            db.add(
                i,
                Some(EventClass::Class1),
                AnalogInputConfig {
                    s_var: StaticAnalogInputVariation::Group30Var5,
                    e_var: EventAnalogInputVariation::Group32Var7,
                    deadband: 0.0,
                },
            );

            db.update(
                i,
                &AnalogInput::new(
                    *dnp3_analog_input_init_value.index(i_usize),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

pub fn update_analog_input_value(db: &mut Database) {
    let dnp3_analog_input_random_update_interval =
        env::var("DNP3_ANALOG_INPUT_RANDOM_UPDATE_INTERVAL")
            .unwrap()
            .parse::<u64>()
            .unwrap_or(0);

    if dnp3_analog_input_random_update_interval > 0 {
        static mut LAST_RUN: Option<Instant> = None;

        // Safely update and access LAST_RUN
        let now = Instant::now();
        let should_run = unsafe {
            if let Some(last_run) = LAST_RUN {
                if now.duration_since(last_run)
                    >= Duration::from_millis(dnp3_analog_input_random_update_interval)
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
                    for analog_input_index in 0..dnp3_analog_input_total {
                        let dnp3_analog_input_value: Vec<Vec<f64>> = serde_json::from_str(
                            env::var("DNP3_ANALOG_INPUT_RANGE").unwrap().as_str(),
                        )
                        .expect("Failed to parse DNP3_ANALOG_INPUT_RANGE");

                        let dnp3_analog_input_low =
                            dnp3_analog_input_value[analog_input_index as usize][0];
                        let dnp3_analog_input_high =
                            dnp3_analog_input_value[analog_input_index as usize][1];

                        let update_value = generate_random_float(
                            dnp3_analog_input_low.into(),
                            dnp3_analog_input_high.into(),
                        );

                        db.update(
                            analog_input_index,
                            &AnalogInput::new(update_value, Flags::ONLINE, get_current_time()),
                            UpdateOptions::detect_event(),
                        );
                    }
                }
            }
        }
    }
}
