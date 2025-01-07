use crate::control_handlers::ExampleControlHandler;
use crate::datapoint_initial::initialize_database;
use crate::outstation::run_outstation;
use crate::outstation_application::ExampleOutstationApplication;
use crate::outstation_config::get_outstation_config;
use crate::outstation_information::ExampleOutstationInformation;
use dnp3::app::{NullListener, RetryStrategy};
use dnp3::serial::{spawn_outstation_serial_2, SerialSettings};
use std::time::Duration;

pub async fn run_serial(
    _serial_port: String,
    _outstation_address: u16,
    _master_address: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    // ANCHOR: create_serial_server
    let outstation = spawn_outstation_serial_2(
        &*_serial_port,
        SerialSettings::default(),
        get_outstation_config(_outstation_address, _master_address),
        RetryStrategy::new(Duration::from_secs(1), Duration::from_secs(60)),
        // customizable trait that controls outstation behavior
        Box::new(ExampleOutstationApplication),
        // customizable trait to receive events about what the outstation is doing
        Box::new(ExampleOutstationInformation),
        // customizable trait to process control requests from the master
        Box::new(ExampleControlHandler),
        NullListener::create(),
    );
    // ANCHOR_END: create_serial_server

    initialize_database(&outstation);

    run_outstation(outstation).await
}
