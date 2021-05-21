use crate::report_sink::BoxedReportSink;

mod impl_future;
mod impl_reporting_future;

#[::pin_project::pin_project]
#[derive(Debug)]
pub struct ReportingFuture<F, R>
where
    R: Send + Sync + 'static,
{
    #[pin]
    inner: F,

    #[pin]
    sink_opt: Option<BoxedReportSink<R>>,
}
