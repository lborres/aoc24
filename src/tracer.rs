use std::env;
use tracing::Level;

use crate::time::CustomTimer;

fn check_level() -> Level {
    let level = env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "INFO".to_string())
        .to_uppercase();

    match level.as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => {
            println!("WARNING: Invalid RUST_LOG value, defaulting to INFO");
            Level::INFO
        }
    }
}

pub fn init() {
    tracing_subscriber::fmt()
        .with_timer(CustomTimer)
        .with_max_level(check_level())
        .with_level(true)
        .compact()
        .init();
}
