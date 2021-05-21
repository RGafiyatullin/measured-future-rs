use std::time::Duration;
use std::time::Instant;

mod impl_aggregating;
mod impl_clone;
mod impl_debug;
mod impl_report_sink;

mod aggregated_report;
pub use aggregated_report::AggregatedReport;

pub const DEFAULT_FLUSH_INTERVAL: Duration = Duration::from_secs(5);

pub struct AggregatingSink<R, S> {
    flush_interval: Duration,
    last_flushed_at: Instant,
    flush_requested: bool,

    current: R,
    pass_to: S,
}
