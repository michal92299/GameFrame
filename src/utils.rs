use slog::{Drain, Logger};
use std::sync::Mutex;

pub fn setup_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    Logger::root(Mutex::new(drain).fuse(), o!())
}

pub type Clock = smithay::utils::Clock<smithay::utils::Monotonic>;
