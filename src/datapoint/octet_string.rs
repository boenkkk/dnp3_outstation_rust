use crate::common_util::generate_random_string;
use dnp3::app::measurement::OctetString;
use dnp3::outstation::database::{
    Add, Database, EventClass, OctetStringConfig, Update, UpdateOptions,
};
use std::env;
use std::time::Duration;
use tokio::time::Instant;

pub fn initial_octet_string(db: &mut Database) {
    let dnp3_octet_string_total = env::var("DNP3_OCTET_STRING_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    if dnp3_octet_string_total > 0 {
        let dnp3_octet_string_init_value: Vec<String> =
            serde_json::from_str(&env::var("DNP3_OCTET_STRING_INIT_VALUE").unwrap())
                .expect("Failed to parse DNP3_OCTET_STRING_INIT_VALUE");

        for i in 0..dnp3_octet_string_total {
            let i_usize: usize = i as usize;

            db.add(i, Some(EventClass::Class1), OctetStringConfig);

            let octet_string = OctetString::new(dnp3_octet_string_init_value[i_usize].as_bytes())
                .expect("Invalid OctetString length");
            db.update(i, &octet_string, UpdateOptions::detect_event());
        }
    }
}

pub fn update_octet_string_value(db: &mut Database) {
    let dnp3_octet_string_random_update_interval =
        env::var("DNP3_OCTET_STRING_RANDOM_UPDATE_INTERVAL")
            .unwrap()
            .parse::<u64>()
            .unwrap_or(0);

    if dnp3_octet_string_random_update_interval > 0 {
        static mut LAST_RUN: Option<Instant> = None;

        // Safely update and access LAST_RUN
        let now = Instant::now();
        let should_run = unsafe {
            if let Some(last_run) = LAST_RUN {
                if now.duration_since(last_run)
                    >= Duration::from_millis(dnp3_octet_string_random_update_interval)
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
            let dnp3_octet_string_total = env::var("DNP3_OCTET_STRING_TOTAL")
                .unwrap()
                .parse::<u16>()
                .unwrap_or(0);

            if dnp3_octet_string_total > 0 {
                let is_random_update = env::var("DNP3_OCTET_STRING_RANDOM_UPDATE")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse::<bool>()
                    .expect("Failed to parse DNP3_OCTET_STRING_RANDOM_UPDATE as a boolean");

                if is_random_update {
                    let dnp3_octet_string_max_length: Vec<usize> =
                        serde_json::from_str(&env::var("DNP3_OCTET_STRING_MAX_LENGTH").unwrap())
                            .expect("Failed to parse DNP3_OCTET_STRING_MAX_LENGTH");

                    for index in 0..dnp3_octet_string_total {
                        let max_length = *dnp3_octet_string_max_length
                            .get(index as usize)
                            .unwrap_or(&8); // Default to 8 if out of bounds
                        let value_string = generate_random_string(max_length);

                        if let Ok(octet_string) = OctetString::new(value_string.as_bytes()) {
                            db.update(index, &octet_string, UpdateOptions::detect_event());
                        }
                    }
                }
            }
        }
    }
}
