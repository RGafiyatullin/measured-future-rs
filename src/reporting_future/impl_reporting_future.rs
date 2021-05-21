use super::*;

impl<F, R> ReportingFuture<F, R>
where
    R: Send + Sync + 'static,
{
    pub fn new(inner: F, sink_opt: Option<BoxedReportSink<R>>) -> Self {
        Self { inner, sink_opt }
    }
}
