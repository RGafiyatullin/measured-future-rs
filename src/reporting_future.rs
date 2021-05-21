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

#[::pin_project::pin_project]
#[derive(Debug)]
pub struct ReportingFuture<F, S> {
    #[pin]
    inner: F,
    #[pin]
    sink: S,
    #[pin]
    flush_interval: Duration,
    #[pin]
    flushed_at: Instant,
    #[pin]
    acc: Option<Acc>,
}

impl<F, S> ReportingFuture<F, S>
where
    S: MetricSink,
{
    pub fn new(inner: F, sink: S) -> Self {
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
        let this = self.project();
        let acc = this.acc.get_mut();
        let inner = this.inner;
        let flushed_at = this.flushed_at.get_mut();
        let flush_interval = this.flush_interval;
        let sink = this.sink.get_mut();

        let () = ACC.with(|tl_acc| std::mem::swap(acc, &mut *tl_acc.borrow_mut()));
        let ret = inner.poll(cx);
        let () = ACC.with(|tl_acc| std::mem::swap(acc, &mut *tl_acc.borrow_mut()));
        assert!(acc.is_some());

        if flushed_at.elapsed() > *flush_interval || ret.is_ready() {
            let start = *flushed_at;
            *flushed_at = Instant::now();

            let report = acc
                .as_mut()
                .expect("Stolen Acc :(")
                .flush(start, *flushed_at);

            sink.report(report);
        }

        ret
    }
}
