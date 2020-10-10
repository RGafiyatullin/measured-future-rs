use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub time: Duration,
    pub overhead: Duration,
    pub scopes: HashMap<&'static str, Scope>,
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub poll: Stats,
    pub comp: Stats,

    pub scopes: HashMap<&'static str, Scope>,
}

#[derive(Debug, Clone)]
pub enum Stats {
    NonEmpty {
        count: usize,
        min: f64,
        mean: f64,
        max: f64,
        sum: f64,
    },
    Empty,
}
impl Default for Stats {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<&[f64]> for Stats {
    fn from(dts: &[f64]) -> Self {
        if dts.is_empty() {
            Self::Empty
        } else {
            Self::NonEmpty {
                count: dts.len(),
                min: ::stat::min(dts).0,
                max: ::stat::max(dts).0,
                mean: ::stat::mean(dts),
                sum: dts.into_iter().sum(),
            }
        }
    }
}
