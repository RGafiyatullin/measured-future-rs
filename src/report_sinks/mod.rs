mod discarding;
pub use discarding::DiscardingSink;

mod dumping_to_stdout;
pub use dumping_to_stdout::DumpingToStdoutSink;

pub mod aggregating;
pub use aggregating::AggregatingSink;

pub mod default;
pub use default::DefaultSink;

pub mod mpsc;
pub use mpsc::MpscSink;
