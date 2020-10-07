use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

use crate::MetricSink;
use crate::storage::STORAGE;

const DEFAULT_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct ReportingFuture<F, S> 
    where S: MetricSink,
{
    inner: Pin<Box<F>>,
    sink: S,
    flush_interval: Duration,
    flushed_at: Instant
}

impl<F, S> ReportingFuture<F, S> 
    where S: MetricSink,
{
    pub fn new(inner: F, sink: S) -> Self {
        let inner = Box::pin(inner);
        Self {
            inner,
            sink,
            flush_interval: DEFAULT_INTERVAL,
            flushed_at: Instant::now(),
        }
    }

    pub fn with_flush_interval(mut self, flush_interval: Duration) -> Self {
        self.flush_interval = flush_interval;

        self
    }
}

impl<F, S> Future for ReportingFuture<F, S>
where
    F: Future,
    S: MetricSink + Unpin,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let reporting_future = self.get_mut();

        let inner_pin = Pin::new(&mut reporting_future.inner);
        let ret = inner_pin.poll(cx);

        let dt = reporting_future.flushed_at.elapsed();
        if dt > reporting_future.flush_interval {
            reporting_future.flushed_at = Instant::now();

            let mut report = STORAGE.with(|storage| storage.borrow_mut().flush());
            report.time = dt;
            
            reporting_future.sink.report(report);
        }

        ret
    }
}

impl<F, S> Drop for ReportingFuture<F, S> 
where
    S: MetricSink,
{
    fn drop(&mut self) {
        let report = STORAGE.with(|storage| storage.borrow_mut().flush());
        self.sink.report(report);
    }
}
