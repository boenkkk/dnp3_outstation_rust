use crate::server::run_server;
use dnp3::link::LinkErrorMode;
use dnp3::tcp::*;
use dotenv::dotenv;
use std::env;

mod config;
mod database;
mod handlers;
#[path = "handler/outstation_application.rs"]
mod outstation_application;
#[path = "handler/outstation_information.rs"]
mod outstation_information;
mod scheduler;
mod server;
mod util;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // Load environment variables from .env file
    dotenv().ok();

    // Get the address from the environment variable or default to "0.0.0.0:777"
    let tcp_server_address = env::var("DNP3_TCP_SERVER_ADDRESS").unwrap().to_string();
    let outstation_address = env::var("DNP3_OUTSTATION_ADDRESS")?.parse()?;
    let master_address = env::var("DNP3_MASTER_ADDRESS")?.parse()?;

    // Run the TCP server
    // Parse the address and start the server
    let server = Server::new_tcp_server(LinkErrorMode::Close, tcp_server_address.parse()?);
    run_server(server, outstation_address, master_address).await?;

    Ok(())
}
