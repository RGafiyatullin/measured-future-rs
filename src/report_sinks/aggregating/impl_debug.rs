use super::*;

use std::fmt;

impl<R, S> fmt::Debug for AggregatingSink<R, S>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Aggregating<{}>", std::any::type_name::<R>()))
            .field("flush_interval", &self.flush_interval)
            .field("last_flushed_at", &self.last_flushed_at)
            .field("current", &"...")
            .finish()
    }
}
