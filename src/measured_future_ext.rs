use std::future::Future;

use crate::measured_future::MeasuredFuture;
use crate::meters::DefaultMeter;

pub trait MeasuredFutureExt: Sized {
    fn measure_with<M>(self, meter: M) -> MeasuredFuture<Self, crate::meters::DefaultMeter>
    where
        M: Into<DefaultMeter>,
    {
        MeasuredFuture::new(self, meter.into())
    }
}
impl<T> MeasuredFutureExt for T where T: Future {}
