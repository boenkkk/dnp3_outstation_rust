use crate::common_util::{generate_random_double_bit, generate_random_int, get_double_bit};
use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{DoubleBitBinaryInput, Flags};
use dnp3::outstation::database::{
    Add, Database, DoubleBitBinaryInputConfig, EventClass, Update, UpdateOptions,
};
pub use std::env;
use std::ops::Index;
use std::time::Duration;
use tokio::time::Instant;

pub fn initial_double_bit_binary_input(db: &mut Database) {
    let dnp3_double_bit_binary_input_total = env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    if dnp3_double_bit_binary_input_total > 0 {
        let dnp3_double_bit_binary_input_init_value: Vec<u8> = serde_json::from_str(
            env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_INIT_VALUE")
                .unwrap()
                .as_str(),
        )
        .expect("Failed to parse DNP3_BINARY_INPUT_INIT_VALUE");
        for i in 0..dnp3_double_bit_binary_input_total {
            let i_usize: usize = i as usize;
            db.add(
                i,
                Some(EventClass::Class1),
                DoubleBitBinaryInputConfig::default(),
            );

            db.update(
                i,
                &DoubleBitBinaryInput::new(
                    get_double_bit(dnp3_double_bit_binary_input_init_value.index(i_usize)),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

pub fn update_double_bit_binary_input(db: &mut Database) {
    let dnp3_double_bit_binary_input_random_update_interval =
        env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_RANDOM_UPDATE_INTERVAL")
            .unwrap()
            .parse::<u64>()
            .unwrap_or(0);

    if dnp3_double_bit_binary_input_random_update_interval > 0 {
        static mut LAST_RUN: Option<Instant> = None;

        // Safely update and access LAST_RUN
        let now = Instant::now();
        let should_run = unsafe {
            if let Some(last_run) = LAST_RUN {
                if now.duration_since(last_run)
                    >= Duration::from_millis(dnp3_double_bit_binary_input_random_update_interval)
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
            let dnp3_double_bit_binary_input_total = env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_TOTAL")
                .unwrap()
                .parse::<u16>()
                .unwrap_or(0);

            if dnp3_double_bit_binary_input_total > 0 {
                let is_random_update = env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_RANDOM_UPDATE")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse::<bool>()
                    .expect(
                        "Failed to parse DNP3_DOUBLE_BIT_BINARY_INPUT_RANDOM_UPDATE as a boolean",
                    );

                if is_random_update {
                    let double_bit_binary_input_index =
                        generate_random_int(0u32, dnp3_double_bit_binary_input_total as u32 - 1)
                            as u16;

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
    }
}
