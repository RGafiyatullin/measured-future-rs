use super::*;

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::meter::Meter;

impl<F, M> Future for MeasuredFuture<F, M>
where
    F: Future,
    M: Meter + Unpin,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let meter = this.meter.get_mut();
        let inner = this.inner;

        let () = meter.enter_poll();

        let ret = inner.poll(cx);
        let is_complete = ret.is_ready();

        let () = meter.leave_poll(is_complete);

        ret
    }
}
