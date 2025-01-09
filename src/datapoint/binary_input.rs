use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{BinaryInput, Flags};
use dnp3::outstation::database::{
    Add, BinaryInputConfig, Database, EventBinaryInputVariation, EventClass,
    StaticBinaryInputVariation, Update, UpdateOptions,
};
use std::env;
use std::ops::Index;

pub fn initial_binary_input(db: &mut Database) {
    let dnp3_binary_input_total = env::var("DNP3_BINARY_INPUT_TOTAL")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    if dnp3_binary_input_total > 0 {
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
}
