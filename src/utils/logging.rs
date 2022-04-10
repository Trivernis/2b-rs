/*
 * snekcloud node based network
 * Copyright (C) 2020 trivernis
 * See LICENSE for more information
 */

use std::str::FromStr;

use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

const DEFAULT_ENV_FILTER: &str = "info,serenity=warn";

/// Initializes tracing
pub fn init_logger() {
    let filter_string =
        std::env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_ENV_FILTER.to_string());
    let env_filter =
        EnvFilter::from_str(&*filter_string).expect("failed to parse env filter string");
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(env_filter)
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .compact()
        .init();
}
