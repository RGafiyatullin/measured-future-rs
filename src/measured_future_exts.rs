use std::future::Future;

use crate::MeasuredFuture;

pub trait MeasuredFutureExts: Future + Sized {
    fn measured(self, key: &'static str) -> MeasuredFuture<Self> {
        MeasuredFuture::new(self, key)
    }
}

impl<F: Future> MeasuredFutureExts for F {}
