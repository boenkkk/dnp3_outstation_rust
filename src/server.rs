// src/server.rs
use crate::config::get_outstation_config;
use crate::handlers::ExampleControlHandler;
// use crate::outstation::{ExampleOutstationApplication, ExampleOutstationInformation};
use dnp3::app::attr::{AttrProp, StringAttr};
// use dnp3::app::measurement::*;
use dnp3::app::NullListener;
use dnp3::outstation::database::*;
// use dnp3::outstation::*;
use dnp3::tcp::*;
use crate::time::{ExampleOutstationApplication, ExampleOutstationInformation};

use dnp3::outstation::OutstationHandle;

pub async fn run_server(mut server: Server) -> Result<(), Box<dyn std::error::Error>> {
    let outstation = server.add_outstation(
        get_outstation_config(),
        Box::new(ExampleOutstationApplication),
        Box::new(ExampleOutstationInformation),
        Box::new(ExampleControlHandler),
        NullListener::create(),
        AddressFilter::Any,
    )?;

    initialize_database(&outstation);

    let _server_handle = server.bind().await?;
    run_outstation(outstation).await
}

async fn run_outstation(
    mut outstation: OutstationHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    outstation.enable().await?;

    loop {}
}

fn initialize_database(outstation: &OutstationHandle) {
    outstation.transaction(|db| {
        for i in 0..3 {
            db.add(
                i,
                Some(EventClass::Class1),
                BinaryInputConfig {
                    s_var: StaticBinaryInputVariation::Group1Var1,
                    e_var: EventBinaryInputVariation::Group2Var2,
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
                    s_var: StaticAnalogInputVariation::Group30Var1,
                    e_var: EventAnalogInputVariation::Group32Var1,
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
