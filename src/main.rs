use crate::serial::run_serial;
use crate::tcp::run_tcp;
use dnp3::link::LinkErrorMode;
use dnp3::tcp::*;
use dotenv::dotenv;
use std::env;

#[path = "util/common_util.rs"]
mod common_util;
#[path = "handler/control_handlers.rs"]
mod control_handlers;
#[path = "datapoint/datapoint_initial.rs"]
mod datapoint_initial;
#[path = "util/dnp3_util.rs"]
mod dnp3_util;
#[path = "config/event_buffer.rs"]
mod event_buffer;
#[path = "outstation/outstation.rs"]
mod outstation;
#[path = "handler/outstation_application.rs"]
mod outstation_application;
#[path = "config/outstation_config.rs"]
mod outstation_config;
#[path = "handler/outstation_information.rs"]
mod outstation_information;
mod scheduler;
#[path = "outstation/serial.rs"]
mod serial;
#[path = "outstation/tcp.rs"]
mod tcp;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // Load environment variables from .env file
    dotenv().ok();

    // Get the address from the environment variable
    let dnp3_outstation_type = env::var("DNP3_OUTSTATION_TYPE").unwrap().to_string();
    let outstation_address = env::var("DNP3_OUTSTATION_ADDRESS")?.parse()?;
    let master_address = env::var("DNP3_MASTER_ADDRESS")?.parse()?;

    if dnp3_outstation_type == "TCP" {
        let tcp_server_address = env::var("DNP3_TCP_SERVER_ADDRESS").unwrap();
        let tcp_server_port = env::var("DNP3_TCP_SERVER_PORT").unwrap();
        let tcp_server_url = format!("{}:{}", tcp_server_address, tcp_server_port);

        // Run the TCP server
        // Parse the address and start the server
        let server = Server::new_tcp_server(LinkErrorMode::Close, tcp_server_url.parse()?);
        run_tcp(server, outstation_address, master_address).await?;
    } else if dnp3_outstation_type == "SERIAL" {
        let dnp3_serial_port = env::var("DNP3_SERIAL_PORT").unwrap().to_string();
        run_serial(dnp3_serial_port, outstation_address, master_address).await?;
    }

    Ok(())
}
