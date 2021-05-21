use crate::report::Report;

use std::sync::mpsc::Sender as SyncSender;

use ::futures::channel::mpsc::Sender as AsyncSender;

pub trait MetricSink {
    fn report(&mut self, report: Report);
}

pub trait MetricSinkInstall: MetricSink {
    fn install(self);
}

impl MetricSink for Box<dyn MetricSink> {
    fn report(&mut self, report: Report) {
        self.as_mut().report(report)
    }
}

impl<S> MetricSinkInstall for S
where
    S: MetricSink + Clone + 'static,
{
    fn install(self) {
        crate::sink::SINK.with(|factory| {
            *factory.borrow_mut() =
                Box::new(crate::sink::factory::MetricSinkFactoryImpl::new(self));
        });
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
