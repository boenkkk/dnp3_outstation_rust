#[path = "analog_input.rs"]
mod analog_input;
#[path = "analog_output.rs"]
mod analog_output;
#[path = "binary_input.rs"]
mod binary_input;
#[path = "binary_output.rs"]
mod binary_output;

#[path = "double_bit_binary_input.rs"]
mod double_bit_binary_input;

use analog_input::initial_analog_input;
use analog_output::initial_analog_output;
use binary_input::initial_binary_input;
use binary_output::initial_binary_output;
use double_bit_binary_input::initial_double_bit_binary_input;

use crate::dnp3_util::get_current_time;
use dnp3::app::attr::{AttrProp, StringAttr};
use dnp3::app::measurement::{Counter, Flags, FrozenCounter};
use dnp3::outstation::database::{
    Add, CounterConfig, Database, EventClass, FrozenCounterConfig, OctetStringConfig, Update,
    UpdateOptions,
};
use dnp3::outstation::OutstationHandle;
use std::env;
use std::ops::Index;

pub fn initialize_database(outstation: &OutstationHandle) {
    outstation.transaction(|db| {
        let _ = db.define_attr(
            AttrProp::default(),
            StringAttr::DeviceManufacturersName.with_value("Step Function I/O"),
        );
        let _ = db.define_attr(
            AttrProp::writable(),
            StringAttr::UserAssignedLocation.with_value("Bend, OR"),
        );
        db.add(0, Some(EventClass::Class1), OctetStringConfig);

        initial_binary_input(db);
        initial_binary_output(db);
        initial_analog_input(db);
        initial_analog_output(db);
        initial_double_bit_binary_input(db);
        initial_counter(db);
        initial_frozen_counter(db);
    });
}

fn initial_counter(db: &mut Database) {
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

fn initial_frozen_counter(db: &mut Database) {
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
