use crate::report_sink::BoxedReportSink;
use crate::report_sink::ReportSink;

pub struct DiscardingSink<R>(std::marker::PhantomData<R>);

impl<R> DiscardingSink<R> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<R> ReportSink<R> for DiscardingSink<R>
where
    R: Send + Sync + 'static,
{
    fn send_report(&mut self, _report: R) {}
    fn flush(&mut self) {}
    fn clone_sink(&mut self) -> BoxedReportSink<R> {
        BoxedReportSink::new(self.clone())
    }
}

impl<T> Clone for DiscardingSink<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Debug for DiscardingSink<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Discard<{}>", std::any::type_name::<T>())
    }
}
