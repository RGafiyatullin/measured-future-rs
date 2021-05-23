pub mod meter;
pub mod meters;

pub mod report_sink;
pub mod report_sinks;

pub mod measured_future;
pub mod reporting_future;

pub mod measured_future_ext;
pub mod reporting_future_ext;

pub mod prelude {
    pub use crate::meter::Meter;

    pub use crate::measured_future_ext::MeasuredFutureExt;
    pub use crate::reporting_future_ext::ReportingFutureExt;

    pub mod report_sink {
        pub use crate::report_sink::install;
    }
}
