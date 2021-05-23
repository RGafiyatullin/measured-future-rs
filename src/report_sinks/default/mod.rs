use super::*;

use ::futures::prelude::*;

use crate::meters::default_meter::DefaultMeterReport;
use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;

mod aggregator;
pub use aggregator::Aggregator;
pub use aggregator::Scope;
pub use aggregator::ScopeProps;

#[derive(Debug, Clone)]
pub struct DefaultSink {
    sink: AggregatingSink<Aggregator, MpscUnboundedSink<Aggregator>>,
}

impl DefaultSink {
    pub fn install() {
        let (mpsc_sink, rx) = MpscUnboundedSink::create();
        let aggregating_sink = AggregatingSink::new(mpsc_sink);

        let sink = Self {
            sink: aggregating_sink,
        };

        let _ = ::tokio::spawn(async move {
            ::futures::pin_mut!(rx);
            while let Some(event) = rx.next().await {
                println!("{:#?}", event)
            }
        });

        let _ = crate::report_sink::install(sink);
    }
}

impl ReportSink<DefaultMeterReport> for DefaultSink {
    fn send_report(&mut self, report: DefaultMeterReport) {
        self.sink.send_report(report);
    }
    fn flush(&mut self) {
        ReportSink::<DefaultMeterReport>::flush(&mut self.sink);
    }
    fn clone_sink(&mut self) -> BoxedReportSink<DefaultMeterReport> {
        BoxedReportSink::new(self.clone())
    }
}
