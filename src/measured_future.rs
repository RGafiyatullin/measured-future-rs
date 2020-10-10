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
    first_poll_at: Option<Instant>,
}

impl<F> MeasuredFuture<F> {
    pub fn new(inner: F, key: &'static str) -> Self {
        let inner = Box::pin(inner);
        Self {
            inner,
            key,
            first_poll_at: None,
        }
    }
}

impl<F> Future for MeasuredFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let measured_future = self.get_mut();

        if measured_future.first_poll_at.is_none() {
            measured_future.first_poll_at = Some(Instant::now());
        }

        let () = ACC.with(|storage_opt| {
            if let Some(ref mut storage) = *storage_opt.borrow_mut() {
                storage.push(measured_future.key);
            }
        });

        let inner_pin = Pin::new(&mut measured_future.inner);

        let t0 = Instant::now();
        let ret = inner_pin.poll(cx);
        let is_ready = ret.is_ready();
        let dt = t0.elapsed();

        let () = ACC.with(|storage_opt| {
            if let Some(ref mut storage) = *storage_opt.borrow_mut() {
                let frame = storage.pop();
                frame.add_poll(dt);

                if is_ready {
                    if let Some(first_poll_at) = measured_future.first_poll_at {
                        frame.add_completion(first_poll_at.elapsed())
                    }
                }
            }
        });

        ret
    }
}
