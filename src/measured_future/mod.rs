mod impl_future;
mod impl_measured_future;

#[::pin_project::pin_project]
#[derive(Debug)]
pub struct MeasuredFuture<F, M> {
    #[pin]
    inner: F,
    #[pin]
    meter: M,
}
