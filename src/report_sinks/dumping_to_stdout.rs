use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;

pub struct DumpingToStdoutSink<R>(std::marker::PhantomData<R>);

impl<R> DumpingToStdoutSink<R> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<R> ReportSink<R> for DumpingToStdoutSink<R>
where
    R: Send + Sync + 'static,
    R: std::fmt::Debug,
{
    fn send_report(&mut self, report: R) {
        println!("[{:?}] {:#?}", self, report);
    }

    fn flush(&mut self) {}

    fn clone_sink(&mut self) -> BoxedReportSink<R> {
        BoxedReportSink::new(self.clone())
    }
}

impl<T> Clone for DumpingToStdoutSink<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Debug for DumpingToStdoutSink<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DumpToStdout<{}>", std::any::type_name::<T>())
    }
}
