use super::*;

impl From<&'static str> for DefaultMeter {
    fn from(key: &'static str) -> Self {
        DefaultMeter::new(key)
    }
}
