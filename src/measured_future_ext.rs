use std::future::Future;

use crate::measured_future::MeasuredFuture;

pub trait MeasuredFutureExt: Sized {
    fn measure_with<M>(self, meter: M) -> MeasuredFuture<Self, M> {
        MeasuredFuture::new(self, meter)
    }
}
impl<T> MeasuredFutureExt for T where T: Future {}
