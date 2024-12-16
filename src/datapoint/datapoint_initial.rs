#[path = "analog_input.rs"]
mod analog_input;
#[path = "analog_output.rs"]
mod analog_output;

use analog_input::initial_analog_input;
use analog_output::initial_analog_output;

use crate::common_util::get_double_bit;
use crate::dnp3_util::get_current_time;
use dnp3::app::attr::{AttrProp, StringAttr};
use dnp3::app::measurement::{
    BinaryInput, BinaryOutputStatus, Counter, DoubleBitBinaryInput, Flags, FrozenCounter,
};
use dnp3::outstation::database::{
    Add, BinaryInputConfig, BinaryOutputStatusConfig, CounterConfig, Database,
    DoubleBitBinaryInputConfig, EventBinaryInputVariation, EventClass, FrozenCounterConfig,
    OctetStringConfig, StaticBinaryInputVariation, Update, UpdateOptions,
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

fn initial_binary_input(db: &mut Database) {
    let dnp3_binary_input_total = env::var("DNP3_BINARY_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let dnp3_binary_input_init_value: Vec<bool> =
        serde_json::from_str(env::var("DNP3_BINARY_INPUT_INIT_VALUE").unwrap().as_str())
            .expect("Failed to parse DNP3_BINARY_INPUT_INIT_VALUE");
    for i in 0..dnp3_binary_input_total {
        let i_usize: usize = i as usize;

        db.add(
            i,
            Some(EventClass::Class1),
            BinaryInputConfig {
                s_var: StaticBinaryInputVariation::Group1Var2,
                e_var: EventBinaryInputVariation::Group2Var3,
            },
        );

        db.update(
            i,
            &BinaryInput::new(
                *dnp3_binary_input_init_value.index(i_usize),
                Flags::ONLINE,
                get_current_time(),
            ),
            UpdateOptions::detect_event(),
        );
    }
}

fn initial_binary_output(db: &mut Database) {
    let dnp3_binary_output_total = env::var("DNP3_BINARY_OUTPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let dnp3_binary_output_init_value: Vec<bool> =
        serde_json::from_str(env::var("DNP3_BINARY_OUTPUT_INIT_VALUE").unwrap().as_str())
            .expect("Failed to parse DNP3_BINARY_INPUT_INIT_VALUE");
    for i in 0..dnp3_binary_output_total {
        let i_usize: usize = i as usize;

        db.add(
            i,
            Some(EventClass::Class1),
            BinaryOutputStatusConfig::default(),
        );

        db.update(
            i,
            &BinaryOutputStatus::new(
                *dnp3_binary_output_init_value.index(i_usize),
                Flags::ONLINE,
                get_current_time(),
            ),
            UpdateOptions::detect_event(),
        );
    }
}

fn initial_double_bit_binary_input(db: &mut Database) {
    let dnp3_double_bit_binary_input_total = env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();
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

fn initial_counter(db: &mut Database) {
    let dnp3_counter_total = env::var("DNP3_COUNTER_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();
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

fn initial_frozen_counter(db: &mut Database) {
    let dnp3_frozen_counter_total = env::var("DNP3_FROZEN_COUNTER_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();
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
