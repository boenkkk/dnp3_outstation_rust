use dnp3::app::attr::{AttrProp, StringAttr};
use dnp3::outstation::database::{Add, AnalogInputConfig, AnalogOutputStatusConfig, BinaryInputConfig, BinaryOutputStatusConfig, CounterConfig, DoubleBitBinaryInputConfig, EventAnalogInputVariation, EventBinaryInputVariation, EventClass, FrozenCounterConfig, OctetStringConfig, StaticAnalogInputVariation, StaticBinaryInputVariation};
use dnp3::outstation::OutstationHandle;

pub fn initialize_database(outstation: &OutstationHandle) {
    outstation.transaction(|db| {
        for i in 0..3 {
            db.add(
                i,
                Some(EventClass::Class1),
                BinaryInputConfig {
                    s_var: StaticBinaryInputVariation::Group1Var2,
                    e_var: EventBinaryInputVariation::Group2Var3,
                },
            );
            db.add(
                i,
                Some(EventClass::Class1),
                DoubleBitBinaryInputConfig::default(),
            );
            db.add(
                i,
                Some(EventClass::Class1),
                BinaryOutputStatusConfig::default(),
            );
            db.add(i, Some(EventClass::Class1), CounterConfig::default());
            db.add(i, Some(EventClass::Class1), FrozenCounterConfig::default());
            db.add(
                i,
                Some(EventClass::Class1),
                AnalogInputConfig {
                    s_var: StaticAnalogInputVariation::Group30Var4,
                    e_var: EventAnalogInputVariation::Group32Var2,
                    deadband: 0.0,
                },
            );
            db.add(
                i,
                Some(EventClass::Class1),
                AnalogOutputStatusConfig::default(),
            );
            db.add(i, Some(EventClass::Class1), OctetStringConfig);
        }

        let _ = db.define_attr(
            AttrProp::default(),
            StringAttr::DeviceManufacturersName.with_value("Step Function I/O"),
        );
        let _ = db.define_attr(
            AttrProp::writable(),
            StringAttr::UserAssignedLocation.with_value("Bend, OR"),
        );
    });
}
