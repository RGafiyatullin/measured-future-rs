use super::*;

impl DefaultMeter {
    pub fn new(key: &'static str) -> Self {
        Self {
            key,
            first_poll_at: None,
            current_poll_at: None,
        }
    }
}
