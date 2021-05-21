use std::future::Future;
// use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::MetricSinkInstall;

#[::pin_project::pin_project]
pub struct InstallingFuture<F, S> {
    #[pin]
    inner: F,

    #[pin]
    sink_opt: Option<S>,
}

impl<F, S> InstallingFuture<F, S> {
    pub fn new(inner: F, sink: S) -> Self {
        // let inner = Box::pin(inner);
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
        let this = self.project();

        if let Some(sink) = this.sink_opt.get_mut().take() {
            sink.install();
        }

        this.inner.poll(cx)
    }
}
