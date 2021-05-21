use std::{borrow::Borrow, future::Future};

use crate::sink::MetricSinkFactory;
use crate::InstallingFuture;
use crate::MetricSink;

pub trait InstallingFutureExts: Future + Sized {
    fn installing_sink<S>(self, sink: S) -> InstallingFuture<Self>
    where
        S: MetricSink + Clone + 'static,
    {
        let factory = crate::sink::factory::MetricSinkFactoryImpl::new(sink);
        let factory = Box::new(factory);
        InstallingFuture::new(self, factory)
    }

    fn installing_current(self) -> InstallingFuture<Self> {
        let factory = crate::sink::SINK.with(|f| MetricSinkFactory::clone(f.borrow().as_ref()));
        InstallingFuture::new(self, factory)
    }
}

impl<F: Future> InstallingFutureExts for F {}
