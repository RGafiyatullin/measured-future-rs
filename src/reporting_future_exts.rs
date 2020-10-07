use std::future::Future;

use crate::MetricSink;
use crate::ReportingFuture;

pub trait ReportingFutureExts: Future + Sized {
    fn report<S>(self, sink: S) -> ReportingFuture<Self, S>
    where
        S: MetricSink,
    {
        ReportingFuture::new(self, sink)
    }
}

impl<F: Future> ReportingFutureExts for F {}
