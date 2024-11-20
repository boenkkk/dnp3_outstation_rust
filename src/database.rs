use std::env;
use dnp3::app::attr::{AttrProp, StringAttr};
use dnp3::outstation::database::{Add, AnalogInputConfig, AnalogOutputStatusConfig, BinaryInputConfig, BinaryOutputStatusConfig, CounterConfig, DoubleBitBinaryInputConfig, EventAnalogInputVariation, EventBinaryInputVariation, EventClass, FrozenCounterConfig, OctetStringConfig, StaticAnalogInputVariation, StaticBinaryInputVariation};
use dnp3::outstation::OutstationHandle;

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

        let dnp3_binary_input_total = env::var("DNP3_BINARY_INPUT_TOTAL").unwrap().parse::<u16>().unwrap();
        for i in 0..dnp3_binary_input_total {
            db.add(
                i,
                Some(EventClass::Class1),
                BinaryInputConfig {
                    s_var: StaticBinaryInputVariation::Group1Var2,
                    e_var: EventBinaryInputVariation::Group2Var3,
                },
            );
        }

        let dnp3_binary_output_total = env::var("DNP3_BINARY_OUTPUT_TOTAL").unwrap().parse::<u16>().unwrap();
        for i in 0..dnp3_binary_output_total {
            db.add(
                i,
                Some(EventClass::Class1),
                BinaryOutputStatusConfig::default(),
            );
        }

        let dnp3_analog_input_total = env::var("DNP3_ANALOG_INPUT_TOTAL").unwrap().parse::<u16>().unwrap();
        for i in 0..dnp3_analog_input_total {
            db.add(
                i,
                Some(EventClass::Class1),
                AnalogInputConfig {
                    s_var: StaticAnalogInputVariation::Group30Var4,
                    e_var: EventAnalogInputVariation::Group32Var2,
                    deadband: 0.0,
                },
            );
        }

        let dnp3_analog_output_total = env::var("DNP3_ANALOG_OUTPUT_TOTAL").unwrap().parse::<u16>().unwrap();
        for i in 0..dnp3_analog_output_total {
            db.add(
                i,
                Some(EventClass::Class1),
                AnalogOutputStatusConfig::default(),
            );
        }

        let dnp3_double_bit_binary_input_total = env::var("DNP3_DOUBLE_BIT_BINARY_INPUT_TOTAL").unwrap().parse::<u16>().unwrap();
        for i in 0..dnp3_double_bit_binary_input_total {
            db.add(
                i,
                Some(EventClass::Class1),
                DoubleBitBinaryInputConfig::default(),
            );
        }

        let dnp3_counter_total = env::var("DNP3_COUNTER_TOTAL").unwrap().parse::<u16>().unwrap();
        for i in 0..dnp3_counter_total {
            db.add(i, Some(EventClass::Class1), CounterConfig::default());
        }

        let dnp3_frozen_counter_total = env::var("DNP3_FROZEN_COUNTER_TOTAL").unwrap().parse::<u16>().unwrap();
        for i in 0..dnp3_frozen_counter_total {
            db.add(i, Some(EventClass::Class1), FrozenCounterConfig::default());
        }
    });
}
