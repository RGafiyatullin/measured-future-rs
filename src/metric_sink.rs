use crate::report::Report;

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
