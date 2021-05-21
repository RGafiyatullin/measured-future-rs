use super::*;

impl<F, M> MeasuredFuture<F, M> {
    pub fn new(inner: F, meter: M) -> Self {
        Self { inner, meter }
    }
}
