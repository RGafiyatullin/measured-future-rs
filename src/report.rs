use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct Report {
    pub time: Duration,
    pub scopes: HashMap<&'static str, Scope>,
}

#[derive(Debug)]
pub struct Scope {
    pub polls: usize,
    pub time: Duration,
    pub scopes: HashMap<&'static str, Scope>,
}
