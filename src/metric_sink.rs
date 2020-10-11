use crate::report::Report;

use ::futures::channel::mpsc;

pub trait MetricSink: Send + Sync + 'static {
    fn report(&mut self, report: Report);
    fn clone_and_box(&self) -> Box<dyn MetricSink>;
}




#[derive(Debug, Clone, Copy)]
pub struct DumpToStdout;

impl MetricSink for DumpToStdout {
    fn report(&mut self, report: Report) {
        println!("=== REPORT ===");
        println!("{:#?}", report);
        println!("=== ====== ===");
    }

    fn clone_and_box(&self) -> Box<dyn MetricSink> {
        Box::new(Self)
    }
}

impl<R> MetricSink for mpsc::Sender<R>
where
    R: From<Report> + Send + Sync + 'static,
{
    fn report(&mut self, report: Report) {
        let report = report.into();
        if let Err(_reason) = self.try_send(report) {
            log::warn!("Failed to send report");
        }
    }

    fn clone_and_box(&self) -> Box<dyn MetricSink> {
        Box::new(self.clone())
    }
}

impl<S> MetricSink for Option<S> where S: MetricSink + Clone {
    fn report(&mut self, report: Report) {
        if let Some(sink) = self {
            sink.report(report)
        }
    }

    fn clone_and_box(&self) -> Box<dyn MetricSink> {
        Box::new(self.clone())
    }
}
