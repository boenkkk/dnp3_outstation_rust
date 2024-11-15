use dnp3::link::LinkErrorMode;
use crate::server::run_server;
use dnp3::tcp::*;

mod config;
mod time;
mod handlers;
mod server;
mod scheduler;
mod util;
mod database;

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