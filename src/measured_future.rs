use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

#[derive(Debug)]
pub struct MeasuredFuture<F> {
    inner: F,
    key: &'static str,
}

impl<F> MeasuredFuture<F> {
    pub fn new(inner: F, key: &'static str) -> Self {
        Self {
            inner,
            key,
        }
    }
}

impl<F> Future for MeasuredFuture<F>
where
    F: Future + Unpin,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let measured_future = self.get_mut();
        let inner_pin = Pin::new(measured_future);
        inner_pin.poll(cx)
    }
}
