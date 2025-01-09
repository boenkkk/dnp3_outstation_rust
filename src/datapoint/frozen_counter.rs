use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{Flags, FrozenCounter};
use dnp3::outstation::database::{
    Add, Database, EventClass, FrozenCounterConfig, Update, UpdateOptions,
};
use std::env;
use std::ops::Index;

pub fn initial_frozen_counter(db: &mut Database) {
    let dnp3_frozen_counter_total = env::var("DNP3_FROZEN_COUNTER_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    if dnp3_frozen_counter_total > 0 {
        let dnp3_frozen_counter_init_value: Vec<u32> =
            serde_json::from_str(env::var("DNP3_FROZEN_COUNTER_INIT_VALUE").unwrap().as_str())
                .expect("Failed to parse DNP3_FROZEN_COUNTER_INIT_VALUE");
        for i in 0..dnp3_frozen_counter_total {
            let i_usize: usize = i as usize;

            db.add(i, Some(EventClass::Class1), FrozenCounterConfig::default());

            db.update(
                i,
                &FrozenCounter::new(
                    *dnp3_frozen_counter_init_value.index(i_usize),
                    Flags::ONLINE,
                    get_current_time(),
                ),
                UpdateOptions::detect_event(),
            );
        }
    }
}
