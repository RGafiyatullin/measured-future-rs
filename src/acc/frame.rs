use std::time::Duration;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Frame {
    pub(super) children: HashMap<&'static str, usize>,
    pub(super) acc: Duration,
    pub(super) polls: usize,
}

impl Frame {
    pub fn empty() -> Self {
        Self {
            children: Default::default(),
            acc: Default::default(),
            polls: 0,
        }
    }

    pub fn add(&mut self, dt: Duration) {
        self.acc = self.acc + dt;
        self.polls = self.polls + 1;
    }

    pub fn reset(&mut self) {
        self.acc = Default::default();
        self.polls = 0;
    }
}
