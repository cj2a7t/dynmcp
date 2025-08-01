use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::provider::global_provider::get_app_config;

pub fn init_logging() -> impl Drop {
    let config = get_app_config().unwrap();

    let file_appender: RollingFileAppender =
        tracing_appender::rolling::daily(config.log.log_dir.clone(), config.log.log_name.clone());
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(true)
        .with_level(true)
        .pretty();

    let json_file_layer = fmt::layer()
        .json()
        .with_writer(file_writer)
        .with_current_span(true)
        .with_span_events(fmt::format::FmtSpan::CLOSE);

    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.log.log_level.clone()));

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(console_layer)
        .with(json_file_layer)
        .init();

    guard
}
