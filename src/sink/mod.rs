use std::cell::RefCell;

pub(crate) mod factory;
pub use factory::MetricSinkFactory;

use crate::MetricSink;

thread_local! {
    pub(crate) static SINK: RefCell<Box<dyn MetricSinkFactory>> =
        RefCell::new(
            Box::new(
                factory::MetricSinkFactoryImpl::new(
                    crate::DiscardReports
                )
            )
        );
}
