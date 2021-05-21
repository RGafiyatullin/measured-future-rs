use super::*;
pub struct BoxedReportSink<R>(Box<dyn ReportSink<R>>)
where
    R: Send + Sync + 'static;

impl<R> BoxedReportSink<R>
where
    R: Send + Sync + 'static,
{
    pub fn new<S>(sink: S) -> Self
    where
        S: ReportSink<R>,
    {
        Self(Box::new(sink))
    }
}

impl<R> AsMut<dyn ReportSink<R>> for BoxedReportSink<R>
where
    R: Send + Sync + 'static,
{
    fn as_mut(&mut self) -> &mut dyn ReportSink<R> {
        self.0.as_mut()
    }
}

impl<R> std::fmt::Debug for BoxedReportSink<R>
where
    R: Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BoxedReportSink<{}>", std::any::type_name::<R>())
    }
}
