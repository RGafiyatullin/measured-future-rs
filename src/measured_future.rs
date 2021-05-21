use std::borrow::Borrow;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use std::time::Instant;

use crate::acc::ACC;

#[::pin_project::pin_project]
#[derive(Debug)]
pub struct MeasuredFuture<F> {
    #[pin]
    inner: F,
    #[pin]
    key: &'static str,
    #[pin]
    first_poll_at: Option<Instant>,
}

impl<F> MeasuredFuture<F> {
    pub fn new(inner: F, key: &'static str) -> Self {
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
        let this = self.project();

        let first_poll_at = this.first_poll_at.get_mut();
        let key = this.key.borrow();
        let inner = this.inner;

        if first_poll_at.is_none() {
            *first_poll_at = Some(Instant::now());
        }

        let () = ACC.with(|storage_opt| {
            if let Some(ref mut storage) = *storage_opt.borrow_mut() {
                storage.push(key);
            }
        });

        let t0 = Instant::now();
        let ret = inner.poll(cx);
        let is_ready = ret.is_ready();
        let dt = t0.elapsed();

        let () = ACC.with(|storage_opt| {
            if let Some(ref mut storage) = *storage_opt.borrow_mut() {
                let frame = storage.pop();
                frame.add_poll(dt);

                if is_ready {
                    if let Some(first_poll_at) = first_poll_at {
                        frame.add_completion(first_poll_at.elapsed())
                    }
                }
            }
        });

        ret
    }
}
