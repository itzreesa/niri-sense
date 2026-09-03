pub mod sense_app;
pub mod integration;
pub mod util;
pub mod config;
pub mod repl;

use tokio;
use crate::sense_app::SenseApp;
use std::env;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use log::error;
use niri_ipc::socket::SOCKET_PATH_ENV;
use tokio::sync::Mutex;
use crate::config::SenseConfig;
use crate::integration::Integration;
use crate::repl::Repl;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let mut logger_builder = env_logger::Builder::new();
    logger_builder.filter_level(log::LevelFilter::Error);
    logger_builder.filter_level(log::LevelFilter::Debug);
    logger_builder.init();

    println!("-- niri sense {}", VERSION);

    let config = SenseConfig::load_or_save_default().unwrap();

    let socket;
    match env::var(SOCKET_PATH_ENV) {
        Ok(s) => socket = s,
        Err(e) => {
            error!("couldn't find NIRI_SOCKET in your env! make sure you are running niri! else, set it manually! {}", e);
            exit(1);
        }
    }

    let integration = Arc::new(Mutex::new(Integration::new(config)));
    let si = integration.clone();

    tokio::spawn(async {
        if let Ok(mut app) = SenseApp::new(socket, si) {
            if !app.setup_stream().is_ok() {
                exit(1);
            }

            app.loop_stream().await;
        }
    });

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut repl = Repl::new(integration.clone());
    repl.run().await;
}
