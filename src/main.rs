use graphql::new_schema;
use log::info;
use log::LevelFilter::Trace;
use server::start_server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::builder().filter(None, Trace).init();

    info!("Here");

    let schema = new_schema();
    start_server(schema, "127.0.0.1:8000")
}
