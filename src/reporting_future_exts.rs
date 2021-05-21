use std::future::Future;

use crate::MetricSink;
use crate::ReportingFuture;

pub trait ReportingFutureExts: Future + Sized {
    fn report_to<S>(self, sink: S) -> ReportingFuture<Self, S>
    where
        S: MetricSink,
    {
        ReportingFuture::new(self, sink)
    }
    fn report_to_installed(self) -> ReportingFuture<Self, Box<dyn MetricSink>> {
        let sink = crate::sink::SINK.with(|factory| factory.borrow().create_metric_sink());
        ReportingFuture::new(self, sink)
    }
}

impl<F: Future> ReportingFutureExts for F {}
