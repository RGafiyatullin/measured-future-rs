use super::*;

use std::collections::HashMap;

use ::futures::prelude::*;

use crate::meters::default_meter::DefaultMeterReport;
use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;

mod aggregator;
use aggregator::Aggregator;
pub use aggregator::Scope;
pub use aggregator::ScopeProps;

#[derive(Debug, Clone)]
pub struct TreemapSink {
    sink: AggregatingSink<Aggregator, MpscUnboundedSink<Aggregator>>,
}

impl TreemapSink {
    pub fn install<S, E, F>(handler: S, is_sink_failure_terminal: F)
    where
        S: Sink<HashMap<&'static str, Scope>, Error = E> + Send + Sync + 'static,
        F: Fn(&E) -> bool + Send + Sync + 'static,
    {
        let (mpsc_sink, rx) = MpscUnboundedSink::create();
        let aggregating_sink = AggregatingSink::new(mpsc_sink);

        let sink = Self {
            sink: aggregating_sink,
        };

        let _ = ::tokio::spawn(async move {
            ::futures::pin_mut!(rx);
            ::futures::pin_mut!(handler);
            while let Some(event) = rx.next().await {
                if let Err(reason) = handler.send(event.sub).await {
                    let is_terminal = is_sink_failure_terminal(&reason);
                    #[cfg(feature = "debug-logs")]
                    log::warn!(
                        "error sending another report [is-terminal: {}]: {:#?}",
                        is_terminal,
                        reason
                    );

                    if is_terminal {
                        break;
                    }
                }
            }
        });

        let _ = crate::report_sink::install(sink);
    }
}

impl ReportSink<DefaultMeterReport> for TreemapSink {
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
