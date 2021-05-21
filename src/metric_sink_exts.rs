use crate::MetricSink;

pub trait MetricSinkInstallExts: MetricSink {
    fn install(self);
}

impl<S> MetricSinkInstallExts for S
where
    S: MetricSink + Clone + 'static,
{
    fn install(self) {
        crate::sink::SINK.with(|factory| {
            *factory.borrow_mut() =
                Box::new(crate::sink::factory::MetricSinkFactoryImpl::new(self));
        });
    }
}
