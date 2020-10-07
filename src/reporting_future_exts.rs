
use std::future::Future;

use crate::ReportingFuture;
use crate::MetricSink;

pub trait ReportingFutureExts: Future + Sized {
    fn report<S>(self, sink: S) -> ReportingFuture<Self, S> 
        where S: MetricSink,
    {
        ReportingFuture::new(self, sink)
    }
}

impl<F: Future> ReportingFutureExts for F {}
