use std::time::Duration;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Frame {
    pub(super) children: HashMap<&'static str, usize>,
    pub(super) poll_durations: Vec<f64>,
    pub(super) completion_durations: Vec<f64>,
}

impl Frame {
    pub fn empty() -> Self {
        Self {
            children: Default::default(),

            poll_durations: Default::default(),
            completion_durations: Default::default(),
        }
    }

    pub fn add_poll(&mut self, dt: Duration) {
        self.poll_durations.push(duration_to_float_seconds(dt))
    }

    pub fn add_completion(&mut self, dt: Duration) {
        self.completion_durations
            .push(duration_to_float_seconds(dt))
    }

    pub fn reset(&mut self) {
        self.completion_durations.truncate(0);
        self.poll_durations.truncate(0);
    }
}

fn duration_to_float_seconds(dt: Duration) -> f64 {
    (dt.as_secs() as f64) + (dt.subsec_nanos() as f64) / 1_000_000_000.0
}
