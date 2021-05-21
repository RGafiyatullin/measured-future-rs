use super::*;

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

impl<F, R> Future for ReportingFuture<F, R>
where
    F: Future,
    R: Send + Sync + 'static,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let inner = this.inner;
        let sink_opt = this.sink_opt.get_mut();

        #[cfg(feature = "debug-logs")]
        log::trace!("installing: {:#?}", sink_opt);

        let sink_taken_opt = crate::report_sink::current::replace(sink_opt.take());

        let ret = inner.poll(cx);

        #[cfg(feature = "debug-logs")]
        log::trace!("uninstalling (in favour of {:#?})", sink_taken_opt);

        *sink_opt = crate::report_sink::current::replace(sink_taken_opt);

        if ret.is_ready() {
            if let Some(sink) = sink_opt.as_mut() {
                let () = sink.as_mut().flush();
            }
        }

        ret
    }
}
