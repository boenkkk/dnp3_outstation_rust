use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{AnalogOutputStatus, Flags};
use dnp3::outstation::database::{
    Add, AnalogOutputStatusConfig, Database, EventClass, Update, UpdateOptions,
};
use std::env;
use std::ops::Index;

pub fn initial_analog_output(db: &mut Database) {
    let dnp3_analog_output_total = env::var("DNP3_ANALOG_OUTPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();
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
