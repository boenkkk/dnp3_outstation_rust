use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{AnalogInput, Flags};
use dnp3::outstation::database::{
    Add, AnalogInputConfig, Database, EventAnalogInputVariation, EventClass,
    StaticAnalogInputVariation, Update, UpdateOptions,
};
use std::env;
use std::ops::Index;

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
                    s_var: StaticAnalogInputVariation::Group30Var4,
                    e_var: EventAnalogInputVariation::Group32Var2,
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
