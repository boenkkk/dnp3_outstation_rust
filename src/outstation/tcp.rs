use dnp3::app::NullListener;
use dnp3::tcp::*;

use crate::control_handlers::ExampleControlHandler;
use crate::datapoint_initial::initialize_database;
use crate::outstation::run_outstation;
use crate::outstation_application::ExampleOutstationApplication;
use crate::outstation_config::get_outstation_config;
use crate::outstation_information::ExampleOutstationInformation;

pub async fn run_tcp(
    mut server: Server,
    _outstation_address: u16,
    _master_address: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let outstation = server.add_outstation(
        get_outstation_config(_outstation_address, _master_address),
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
