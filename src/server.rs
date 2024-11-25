use std::sync::Arc;

use dnp3::app::NullListener;
use dnp3::outstation::OutstationHandle;
use dnp3::tcp::*;

use crate::config::get_outstation_config;
use crate::control_handlers::ExampleControlHandler;
use crate::database::initialize_database;
use crate::outstation_application::ExampleOutstationApplication;
use crate::outstation_information::ExampleOutstationInformation;
use crate::scheduler::run_scheduler;

pub async fn run_server(
    mut server: Server,
    _outstation_address: u16,
    _master_address: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let outstation = server.add_outstation(
        get_outstation_config(1, 2),
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

    // Wrap the `OutstationHandle` in an `Arc` to share ownership
    let outstation_arc = Arc::new(outstation);

    // Pass the `Arc` to `run_scheduler`
    run_scheduler(outstation_arc);
    loop {}
}
