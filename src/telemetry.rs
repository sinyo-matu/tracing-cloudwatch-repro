use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{fmt::MakeWriter, prelude::*};
use tracing_subscriber::{EnvFilter, Registry};
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    let cw_client = rusoto_logs::CloudWatchLogsClient::new(rusoto_core::Region::ApNortheast1);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(sentry_tracing::layer())
        .with(
            tracing_cloudwatch::layer()
                .with_client(
                    cw_client,
                    tracing_cloudwatch::ExportConfig::default()
                        .with_batch_size(5)
                        .with_interval(std::time::Duration::from_secs(1))
                        .with_log_group_name("tracing-cloudwatch")
                        .with_log_stream_name("stream-1"),
                )
                .with_code_location(true)
                .with_target(false),
        )
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}
