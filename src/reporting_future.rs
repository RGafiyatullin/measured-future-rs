use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

// use crate::storage::STORAGE;
use crate::MetricSink;

use crate::acc::Acc;
use crate::acc::ACC;

const DEFAULT_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct ReportingFuture<F, S>
where
    S: MetricSink,
{
    inner: Pin<Box<F>>,
    sink: S,
    flush_interval: Duration,
    flushed_at: Instant,
    acc: Option<Acc>,
}

impl<F, S> ReportingFuture<F, S>
where
    S: MetricSink,
{
    pub fn new(inner: F, sink: S) -> Self {
        let inner = Box::pin(inner);
        Self {
            inner,
            sink,
            flush_interval: DEFAULT_INTERVAL,
            flushed_at: Instant::now(),
            acc: Some(Acc::empty()),
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

        let acc_mut = &mut reporting_future.acc;

        let inner_mut = &mut reporting_future.inner;
        let inner_pin = Pin::new(inner_mut);

        let () = ACC.with(|acc| std::mem::swap(acc_mut, &mut *acc.borrow_mut()));
        let ret = inner_pin.poll(cx);
        let () = ACC.with(|acc| std::mem::swap(acc_mut, &mut *acc.borrow_mut()));
        assert!(reporting_future.acc.is_some());

        if reporting_future.flushed_at.elapsed() > reporting_future.flush_interval || ret.is_ready() {
            let start = reporting_future.flushed_at;
            reporting_future.flushed_at = Instant::now();
            let report = reporting_future
                .acc
                .as_mut()
                .expect("Stolen Acc :(")
                .flush(start, reporting_future.flushed_at);

            reporting_future.sink.report(report);
        }

        ret
    }
}
