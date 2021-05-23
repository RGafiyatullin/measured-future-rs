use std::fmt;

use ::futures::channel::mpsc;

use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;

pub struct MpscUnboundedSink<R>(mpsc::UnboundedSender<R>);

impl<R> MpscUnboundedSink<R>
where
    R: Send + Sync + 'static,
{
    pub fn create() -> (Self, mpsc::UnboundedReceiver<R>) {
        let (tx, rx) = mpsc::unbounded();
        (Self(tx), rx)
    }
    pub fn from_tx(tx: mpsc::UnboundedSender<R>) -> Self {
        Self(tx)
    }
}

impl<R> Clone for MpscUnboundedSink<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R> fmt::Debug for MpscUnboundedSink<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}

impl<R> ReportSink<R> for MpscUnboundedSink<R>
where
    R: Send + Sync + 'static,
{
    fn send_report(&mut self, report: R) {
        if let Err(_reason) = self.0.unbounded_send(report) {
            #[cfg(feature = "debug-logs")]
            log::error!("{:?}: failed to send_report: {}", self, _reason);
        }
    }

    fn flush(&mut self) {}

    fn clone_sink(&mut self) -> BoxedReportSink<R> {
        BoxedReportSink::new(self.clone())
    }
}
