use core::str::FromStr;
use std::env;
use tracing_subscriber::{
    filter::EnvFilter,
    layer::SubscriberExt,
    registry,
    Layer,
};

pub const LOG_FILTER: &str = "RUST_LOG";
pub const HUMAN_LOGGING: &str = "HUMAN_LOGGING";

pub fn init_logging() {
    let filter = match env::var_os(LOG_FILTER) {
        Some(_) => {
            EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` provided")
        }
        None => EnvFilter::new("info"),
    };

    let human_logging = env::var_os(HUMAN_LOGGING)
        .map(|s| {
            bool::from_str(s.to_str().unwrap())
                .expect("Expected `true` or `false` to be provided for `HUMAN_LOGGING`")
        })
        .unwrap_or(true);

    let layer = tracing_subscriber::fmt::Layer::default().with_writer(std::io::stderr);

    let fmt = if human_logging {
        // use pretty logs
        layer
            .with_ansi(true)
            .with_level(true)
            .with_line_number(true)
            .boxed()
    } else {
        // use machine parseable structured logs
        layer
            // disable terminal colors
            .with_ansi(false)
            .with_level(true)
            .with_line_number(true)
            // use json
            .json()
            .boxed()
    };

    let subscriber = registry::Registry::default() // provide underlying span data store
        .with(filter) // filter out low-level debug tracing (eg tokio executor)
        .with(fmt); // log to stdout

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting global default failed");
}
