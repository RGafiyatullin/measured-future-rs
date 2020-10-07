use std::future::Future;
// use std::marker::Unpin;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use std::time::Instant;

use crate::acc::ACC;

#[derive(Debug)]
pub struct MeasuredFuture<F> {
    inner: Pin<Box<F>>,
    key: &'static str,
}

impl<F> MeasuredFuture<F> {
    pub fn new(inner: F, key: &'static str) -> Self {
        let inner = Box::pin(inner);
        Self { inner, key }
    }
}

impl<F> Future for MeasuredFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let measured_future = self.get_mut();

        let () = ACC.with(|storage_opt| {
            if let Some(ref mut storage) = *storage_opt.borrow_mut() {
                storage.push(measured_future.key);
            }
        });

        let inner_pin = Pin::new(&mut measured_future.inner);

        let t0 = Instant::now();
        let ret = inner_pin.poll(cx);
        let dt = t0.elapsed();

        let () = ACC.with(|storage_opt| {
            if let Some(ref mut storage) = *storage_opt.borrow_mut() {
                storage.pop().add(dt);
            }
        });

        ret
    }
}
