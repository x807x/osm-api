use chrono::{Local, SecondsFormat};
use colored::Colorize;
use log::{error, info, LevelFilter};
use osm_api::prelude::*;
use std::io::Write;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            let time = Local::now()
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .as_str()
                .bright_blue();
            let level = record.level().as_str();
            let colored_level = match record.level().to_level_filter() {
                LevelFilter::Info => level.green(),
                LevelFilter::Warn => level.yellow(),
                LevelFilter::Error => level.red(),
                _ => level.into(),
            };
            writeln!(buf, "{} [{}] - {}", time, colored_level, record.args(),)
        })
        .filter(None, LevelFilter::Debug)
        .init();
    match get_versions().await {
        Ok(result) => info!("Success!\n{:#?}", result),
        Err(e) => error!("Error: {}", e),
    }

    match get_capabilities().await {
        Ok(result) => info!("Success!\n{:#?}", result),
        Err(e) => error!("Error: {}", e),
    }
}
