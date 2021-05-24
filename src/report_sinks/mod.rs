mod discarding;
pub use discarding::DiscardingSink;

mod dumping_to_stdout;
pub use dumping_to_stdout::DumpingToStdoutSink;

pub mod aggregating;
pub use aggregating::AggregatingSink;

pub mod treemap;
pub use treemap::TreemapSink;

#[deprecated(since = "0.4.2")]
pub mod default;
#[deprecated(since = "0.4.2")]
pub use default::DefaultSink;

pub mod mpsc_bounded;
pub use mpsc_bounded::MpscBoundedSink;

pub mod mpsc_unbounded;
pub use mpsc_unbounded::MpscUnboundedSink;
