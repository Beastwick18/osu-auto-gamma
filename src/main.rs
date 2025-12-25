use std::error::Error;

use env_logger::Env;
use log::error;

use crate::app::App;

mod app;
mod config;
mod tosu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::new().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    if let Err(err) = App::new().run().await {
        error!("{err:?}");
    }

    Ok(())
}
