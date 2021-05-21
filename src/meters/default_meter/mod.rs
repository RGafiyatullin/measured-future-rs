use std::time::Duration;
use std::time::Instant;

mod default_meter_report;
pub use default_meter_report::DefaultMeterReport;

mod impl_default_meter;
mod impl_meter;

pub struct DefaultMeter {
    key: &'static str,
    first_poll_at: Option<Instant>,
    current_poll_at: Option<Instant>,
}
