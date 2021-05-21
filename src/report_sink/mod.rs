mod boxed;
pub use boxed::BoxedReportSink;

pub(crate) mod current;

pub use current::install;

pub trait ReportSink<R>: std::fmt::Debug + Send + Sync + 'static
where
    R: Send + Sync + 'static,
{
    fn send_report(&mut self, report: R);
    fn flush(&mut self);

    fn clone_sink(&mut self) -> BoxedReportSink<R>;
}
