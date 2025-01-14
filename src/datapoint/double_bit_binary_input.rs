use crate::common_util::get_double_bit;
use crate::dnp3_util::get_current_time;
use dnp3::app::measurement::{DoubleBitBinaryInput, Flags};
use dnp3::outstation::database::{
    Add, Database, DoubleBitBinaryInputConfig, EventClass, Update, UpdateOptions,
};
pub use std::env;
use std::ops::Index;

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
