use std::future::Future;
// use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::sink::MetricSinkFactory;

#[::pin_project::pin_project]
pub struct InstallingFuture<F> {
    #[pin]
    inner: F,

    #[pin]
    sink_factory_opt: Option<Box<dyn MetricSinkFactory>>,
}

impl<F> InstallingFuture<F> {
    pub fn new(inner: F, sink_factory: Box<dyn MetricSinkFactory>) -> Self {
        let sink_factory_opt = Some(sink_factory);
        Self {
            inner,
            sink_factory_opt,
        }
    }
}

impl<F> Future for InstallingFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if let Some(sink_factory) = this.sink_factory_opt.get_mut().take() {
            let _ = crate::sink::SINK.with(|tl_factory| tl_factory.replace(sink_factory));
        }

        this.inner.poll(cx)
    }
}
