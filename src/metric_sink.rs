use crate::report::Report;

use std::sync::mpsc::Sender as SyncSender;

use ::futures::channel::mpsc::Sender as AsyncSender;

pub trait MetricSink: Sized {
    fn report(&mut self, report: Report);
}

pub struct DumpToStdout;

impl MetricSink for DumpToStdout {
    fn report(&mut self, report: Report) {
        println!("=== REPORT ===");
        println!("{:#?}", report);
        println!("=== ====== ===");
    }
}

impl<R> MetricSink for SyncSender<R>
where
    R: From<Report>,
{
    fn report(&mut self, report: Report) {
        let report = report.into();
        if let Err(_reason) = self.send(report) {
            log::warn!("Failed to send report");
        }
    }
}

impl<R> MetricSink for AsyncSender<R>
where
    R: From<Report>,
{
    fn report(&mut self, report: Report) {
        let report = report.into();
        if let Err(_reason) = self.try_send(report) {
            log::warn!("Failed to send report");
        }
    }
}
