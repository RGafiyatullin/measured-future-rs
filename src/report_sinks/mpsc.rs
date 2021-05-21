use std::fmt;

use ::futures::channel::mpsc;

use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;

pub struct MpscSink<R>(mpsc::Sender<R>);

impl<R> MpscSink<R>
where
    R: Send + Sync + 'static,
{
    pub fn create(buf_size: usize) -> (Self, mpsc::Receiver<R>) {
        let (tx, rx) = mpsc::channel(buf_size);
        (Self(tx), rx)
    }
    pub fn from_tx(tx: mpsc::Sender<R>) -> Self {
        Self(tx)
    }
}

impl<R> Clone for MpscSink<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R> fmt::Debug for MpscSink<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}

impl<R> ReportSink<R> for MpscSink<R>
where
    R: Send + Sync + 'static,
{
    fn send_report(&mut self, report: R) {
        if let Err(_reason) = self.0.try_send(report) {
            #[cfg(feature = "debug-logs")]
            log::error!("{:?}: failed to send_report: {}", self, _reason);
        }
    }

    fn flush(&mut self) {
        // XXX: well this one is not too obvious for me
    }

    fn clone_sink(&mut self) -> BoxedReportSink<R> {
        BoxedReportSink::new(self.clone())
    }
}
