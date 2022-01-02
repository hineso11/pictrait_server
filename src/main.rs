use graphql::new_schema;
use log::info;
use log::LevelFilter::{Info};
use server::start_server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::builder().filter(None, Info).init();
    info!("Starting up...");

    let schema = new_schema();
    start_server(schema, "127.0.0.1:8000")
}
