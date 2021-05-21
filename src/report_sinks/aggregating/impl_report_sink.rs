use super::*;

use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;

impl<E, R, S> ReportSink<E> for AggregatingSink<R, S>
where
    E: Send + Sync + 'static,
    R: Send + Sync + 'static,
    R: Default,
    R: AggregatedReport<E>,
    S: ReportSink<R>,
    S: Clone,
{
    fn send_report(&mut self, report: E) {
        self.current.add(report);
        if self.flush_requested && self.current.is_complete() {
            self.flush();
            self.flush_requested = false;
        }
    }

    fn flush(&mut self) {
        if self.current.is_complete() {
            let report = std::mem::replace(&mut self.current, Default::default());
            self.pass_to.send_report(report);
            self.pass_to.flush();
        } else {
            self.flush_requested = true;
        }
    }

    fn clone_sink(&mut self) -> BoxedReportSink<E> {
        BoxedReportSink::new(self.clone())
    }
}
