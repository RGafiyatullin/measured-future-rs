use super::*;

#[derive(Debug, Clone, Copy)]
pub enum DefaultMeterReport {
    Enter(&'static str),
    SinglePoll(Duration),
    Completion(Duration),
    Leave(&'static str),
}
