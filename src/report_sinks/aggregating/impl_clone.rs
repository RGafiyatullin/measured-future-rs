use super::*;

impl<R, S> Clone for AggregatingSink<R, S>
where
    R: Default,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            flush_interval: self.flush_interval,
            last_flushed_at: self.last_flushed_at,
            flush_requested: false,

            current: Default::default(),
            pass_to: self.pass_to.to_owned(),
        }
    }
}
