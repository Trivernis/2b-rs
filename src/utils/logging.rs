/*
 * snekcloud node based network
 * Copyright (C) 2020 trivernis
 * See LICENSE for more information
 */

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::Local;
use colored::*;
use log::{Level, LevelFilter};

/// Initializes the env_logger with a custom format
/// that also logs the thread names
pub fn init_logger() {
    let log_dir = PathBuf::from(dotenv::var("LOG_DIR").unwrap_or("logs".to_string()));

    if !log_dir.exists() {
        fs::create_dir(&log_dir).expect("failed to create log dir");
    }
    fern::Dispatch::new()
        .format(|out, message, record| {
            let color = get_level_style(record.level());
            let mut target = record.target().to_string();
            target.truncate(39);

            out.finish(format_args!(
                "{:<40}| {} {}: {}",
                target.dimmed().italic(),
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record
                    .level()
                    .to_string()
                    .to_lowercase()
                    .as_str()
                    .color(color),
                message
            ))
        })
        .level(
            log::LevelFilter::from_str(
                std::env::var("RUST_LOG")
                    .unwrap_or("info".to_string())
                    .as_str(),
            )
            .unwrap_or(LevelFilter::Info),
        )
        .level_for("tokio", log::LevelFilter::Info)
        .level_for("tracing", log::LevelFilter::Warn)
        .level_for("serenity", log::LevelFilter::Warn)
        .level_for("rustls", log::LevelFilter::Warn)
        .level_for("h2", log::LevelFilter::Warn)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("tungstenite", log::LevelFilter::Warn)
        .level_for("hyper", log::LevelFilter::Warn)
        .level_for("async_tungstenite", log::LevelFilter::Warn)
        .level_for("tokio_util", log::LevelFilter::Warn)
        .level_for("want", log::LevelFilter::Warn)
        .level_for("mio", log::LevelFilter::Warn)
        .level_for("songbird", log::LevelFilter::Warn)
        .level_for("html5ever", log::LevelFilter::Warn)
        .level_for("scraper", log::LevelFilter::Warn)
        .level_for("html5ever", log::LevelFilter::Warn)
        .level_for("cssparser", log::LevelFilter::Warn)
        .level_for("selectors", log::LevelFilter::Warn)
        .level_for("matches", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(
            fern::log_file(log_dir.join(PathBuf::from(format!(
                "{}.log",
                Local::now().format("%Y-%m-%d"),
            ))))
            .expect("failed to create log file"),
        )
        .apply()
        .expect("failed to init logger");
}

fn get_level_style(level: Level) -> colored::Color {
    match level {
        Level::Trace => colored::Color::Magenta,
        Level::Debug => colored::Color::Blue,
        Level::Info => colored::Color::Green,
        Level::Warn => colored::Color::Yellow,
        Level::Error => colored::Color::Red,
    }
}
