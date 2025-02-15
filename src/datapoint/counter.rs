use crate::common_util::generate_random_int;
use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{Counter, Flags};
use dnp3::outstation::database::{Add, CounterConfig, Database, EventClass, Update, UpdateOptions};
use std::env;
use std::ops::Index;
use std::time::Duration;
use tokio::time::Instant;

pub fn initial_counter(db: &mut Database) {
    let dnp3_counter_total = env::var("DNP3_COUNTER_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    if dnp3_counter_total > 0 {
        let dnp3_counter_init_value: Vec<u32> =
            serde_json::from_str(env::var("DNP3_COUNTER_INIT_VALUE").unwrap().as_str())
                .expect("Failed to parse DNP3_COUNTER_INIT_VALUE");
        for i in 0..dnp3_counter_total {
            let i_usize: usize = i as usize;

            db.add(i, Some(EventClass::Class1), CounterConfig::default());

            db.update(
                i,
                &Counter::new(
                    *dnp3_counter_init_value.index(i_usize),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}

pub fn update_counter(db: &mut Database) {
    let dnp3_counter_random_update_interval = env::var("DNP3_COUNTER_RANDOM_UPDATE_INTERVAL")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(0);

    if dnp3_counter_random_update_interval > 0 {
        static mut LAST_RUN: Option<Instant> = None;

        // Safely update and access LAST_RUN
        let now = Instant::now();
        let should_run = unsafe {
            if let Some(last_run) = LAST_RUN {
                if now.duration_since(last_run)
                    >= Duration::from_millis(dnp3_counter_random_update_interval)
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
                    let counter_index =
                        generate_random_int(0u32, dnp3_counter_total as u32 - 1) as u16;

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
    }
}
