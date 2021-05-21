use std::future::Future;
// use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::MetricSinkInstall;

pub struct InstallingFuture<F, S> {
    inner: Pin<Box<F>>,
    sink_opt: Option<S>,
}

impl<F, S> InstallingFuture<F, S> {
    pub fn new(inner: F, sink: S) -> Self {
        let inner = Box::pin(inner);
        let sink_opt = Some(sink);
        Self { inner, sink_opt }
    }
}

impl<F, S> Future for InstallingFuture<F, S>
where
    F: Future,
    S: MetricSinkInstall + Unpin,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let installing_future = self.get_mut();
        if let Some(sink) = installing_future.sink_opt.take() {
            sink.install();
        }

        let inner_pin = Pin::new(&mut installing_future.inner);

        inner_pin.poll(cx)
    }
}
