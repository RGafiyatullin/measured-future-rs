use super::*;

impl<R, S> AggregatingSink<R, S>
where
    R: Default,
{
    pub fn new(pass_to: S) -> Self {
        Self {
            flush_interval: DEFAULT_FLUSH_INTERVAL,
            last_flushed_at: Instant::now(),
            current: Default::default(),
            flush_requested: false,
            pass_to,
        }
    }
}

impl<R, S> AggregatingSink<R, S> {
    pub fn with_flush_interval(self, flush_interval: Duration) -> Self {
        Self {
            flush_interval,
            ..self
        }
    }
}
