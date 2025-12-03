use tracing_subscriber::{self, fmt};

pub fn init_tracing() {
    fmt::Subscriber::builder()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_timer(fmt::time::ChronoUtc::new("[%H:%M:%S]".into()))
        .init();
}
