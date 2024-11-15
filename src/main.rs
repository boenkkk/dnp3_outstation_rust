use dnp3::link::LinkErrorMode;
// src/main.rs
// use crate::config::get_outstation_config;
// use crate::handlers::ExampleControlHandler;
// use crate::outstation::{ExampleOutstationApplication, ExampleOutstationInformation};
use crate::server::run_server;
use dnp3::tcp::*;

mod config;
// mod handlers;
// mod outstation;
// mod server;
// mod time;
mod time;
mod handlers;
mod server;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    run_tcp_server().await?;

    Ok(())
}

async fn run_tcp_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:777";
    let server = Server::new_tcp_server(LinkErrorMode::Close, addr.parse()?);
    run_server(server).await
}