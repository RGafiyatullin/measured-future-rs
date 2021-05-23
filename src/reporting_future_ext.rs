use std::future::Future;

use crate::meters::default_meter::DefaultMeterReport;
use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;
use crate::reporting_future::ReportingFuture;

pub trait ReportingFutureExt: Sized {
    fn report_to<S, R>(self, sink: S) -> ReportingFuture<Self, R>
    where
        R: Send + Sync + 'static,
        S: ReportSink<R>,
    {
        let sink = BoxedReportSink::new(sink);
        ReportingFuture::new(self, Some(sink))
    }
    fn report_to_current(self) -> ReportingFuture<Self, DefaultMeterReport> {
        let sink_opt = crate::report_sink::current::with::<DefaultMeterReport, _, _>(|sink_opt| {
            sink_opt.map(|sink| sink.clone_sink())
        });
        ReportingFuture::new(self, sink_opt)
    }
}
impl<T> ReportingFutureExt for T where T: Future {}
