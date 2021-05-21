use super::*;

#[derive(Debug, Clone)]
pub(crate) struct MetricSinkFactoryImpl<S> {
    sink: S,
}

impl<S> MetricSinkFactoryImpl<S> {
    pub fn new(sink: S) -> Self {
        Self { sink }
    }
}

impl<S> MetricSinkFactoryImpl<S>
where
    S: MetricSink + Clone + 'static,
{
    fn create_metric_sink_impl(&self) -> Box<dyn MetricSink> {
        Box::new(self.sink.clone())
    }
}

pub trait MetricSinkFactory: Send + Sync + 'static {
    fn create_metric_sink(&self) -> Box<dyn MetricSink>;
    fn clone(&self) -> Box<dyn MetricSinkFactory>;
}
impl<S> MetricSinkFactory for MetricSinkFactoryImpl<S>
where
    S: MetricSink + Clone + 'static,
{
    fn create_metric_sink(&self) -> Box<dyn MetricSink> {
        self.create_metric_sink_impl()
    }
    fn clone(&self) -> Box<dyn MetricSinkFactory> {
        Box::new(Clone::clone(self))
    }
}
