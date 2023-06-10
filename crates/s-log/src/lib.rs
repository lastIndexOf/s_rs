use log::{error, info, log, Level};

mod tracing_log;

pub use tracing_log::{hello_tracing, init_tracing};

pub fn init_env_log() {
    env_logger::init();
}

pub fn hello() {
    info!("Hello, world!");
    log!(Level::Debug, "Info Message");
    error!("Error Message");
}
