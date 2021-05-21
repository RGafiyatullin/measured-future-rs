use crate::report::Report;

use std::sync::mpsc::Sender as SyncSender;

use ::futures::channel::mpsc::Sender as AsyncSender;

pub trait MetricSink: Send + Sync + 'static {
    fn report(&mut self, report: Report);
}

impl MetricSink for Box<dyn MetricSink> {
    fn report(&mut self, report: Report) {
        self.as_mut().report(report)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DumpToStdout;

#[derive(Debug, Clone, Copy)]
pub struct DiscardReports;

impl MetricSink for DumpToStdout {
    fn report(&mut self, report: Report) {
        println!("=== REPORT ===");
        println!("{:#?}", report);
        println!("=== ====== ===");
    }
}

impl MetricSink for DiscardReports {
    fn report(&mut self, _report: Report) {}
}

impl<R> MetricSink for AsyncSender<R>
where
    R: From<Report> + Send + Sync + 'static,
{
    fn report(&mut self, report: Report) {
        let report = report.into();
        if let Err(_reason) = self.try_send(report) {
            log::warn!("Failed to send report");
        }
    }
}
