use tracing::info;

use crate::telemetry::{get_subscriber, init_subscriber};

mod telemetry;

fn main() {
    let _guard = sentry::init((
        "foofoofoo",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some("production".into()),
            traces_sample_rate: 0.3,
            ..sentry::ClientOptions::default()
        },
    ));
    let subscriber = get_subscriber("test".into(), "info".into(), std::io::stdout);
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            init_subscriber(subscriber);
            info!("Hello world!")
        });
}
