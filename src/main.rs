mod consts;
mod message;
mod server;
mod storage;
mod utils;

use crate::{server::Server, storage::SimpleStorage, utils::HRGenerator};

#[tokio::main]
async fn main() {
    env_logger::init();
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("hi");

    let server: Server<SimpleStorage, HRGenerator> = Server::new();
    server.start().await
}
