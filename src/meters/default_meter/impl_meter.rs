use super::*;

use crate::meter::Meter;

impl Meter for DefaultMeter {
    fn enter_poll(&mut self) {
        let now = Instant::now();
        if self.first_poll_at.is_none() {
            self.first_poll_at = Some(now);
        }

        assert!(
            self.current_poll_at.replace(now).is_none(),
            "self.current_poll_at wasn't empty when enter_poll was invoked"
        );

        crate::report_sink::current::with::<DefaultMeterReport, _, _>(|sink_opt| {
            if let Some(sink) = sink_opt {
                sink.send_report(DefaultMeterReport::Enter(self.key));
            }
        });
    }
    fn leave_poll(&mut self, is_complete: bool) {
        let single_poll_report_opt = self
            .current_poll_at
            .take()
            .map(|current_poll_at| DefaultMeterReport::SinglePoll(current_poll_at.elapsed()));

        let completion_report_opt = self
            .first_poll_at
            .filter(|_| is_complete)
            .map(|first_poll_at| DefaultMeterReport::Completion(first_poll_at.elapsed()));

        crate::report_sink::current::with::<DefaultMeterReport, _, _>(|sink_opt| {
            #[cfg(feature = "debug-logs")]
            log::trace!("sink_opt: {:#?}", sink_opt);

            if let Some(sink) = sink_opt {
                if let Some(single_poll_report) = single_poll_report_opt {
                    let () = sink.send_report(single_poll_report.into());
                }
                if let Some(completion_report) = completion_report_opt {
                    sink.send_report(completion_report.into());
                }
                sink.send_report(DefaultMeterReport::Leave(self.key));
            }
        });
    }
}
