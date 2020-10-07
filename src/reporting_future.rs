use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

#[derive(Debug)]
pub struct ReportingFuture<F, S> {
    inner: F,
    sink: S,
}

impl<F, S> ReportingFuture<F, S> {
    pub fn new(inner: F, sink: S) -> Self {
        Self {
            inner,
            sink,
        }
    }
}

impl<F, S> Future for ReportingFuture<F, S>
where
    F: Future + Unpin,
    S: Unpin,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let measured_future = self.get_mut();
        let inner_pin = Pin::new(measured_future);
        inner_pin.poll(cx)
    }
}
