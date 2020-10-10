use std::time::Duration;
use std::time::Instant;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Report {
    pub start: Instant,
    pub end: Instant,
    pub overhead: Duration,
    pub scopes: HashMap<&'static str, Scope>,
}

impl Report {
    pub fn empty() -> Self {
        Self {
            start: Instant::now(),
            end: Instant::now(),
            overhead: Duration::from_nanos(0),
            scopes: Default::default(),
        }
    }

    pub fn merge(self, other: Self) -> Self {
        let t = Instant::now();
        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };
        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };
        let scopes = merge_scope_maps(self.scopes, other.scopes);
        Self {
            start,
            end,
            overhead: self.overhead + other.overhead + t.elapsed(),
            scopes,
        }
    }
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

impl Scope {
    pub fn merge(self, other: Self) -> Self {
        let l_scopes = self.scopes;
        let r_scopes = other.scopes;

        let scopes = merge_scope_maps(l_scopes, r_scopes);

        Self {
            poll: self.poll.merge(other.poll),
            comp: self.comp.merge(other.comp),
            scopes,
        }
    }
}

fn merge_scope_maps(
    left: HashMap<&'static str, Scope>,
    right: HashMap<&'static str, Scope>,
) -> HashMap<&'static str, Scope> {
    left.into_iter().chain(right.into_iter()).fold(
        HashMap::<&'static str, Scope>::new(),
        |mut acc, (k, v)| {
            let merged = if let Some(existing) = acc.remove(&k) {
                existing.merge(v)
            } else {
                v
            };
            acc.insert(k, merged);

            acc
        },
    )
}

impl Stats {
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Empty, right) => right.clone(),
            (left, Self::Empty) => left.clone(),
            (
                Self::NonEmpty {
                    count: l_count,
                    min: l_min,
                    max: l_max,
                    mean: _,
                    sum: l_sum,
                },
                Self::NonEmpty {
                    count: r_count,
                    min: r_min,
                    max: r_max,
                    mean: _,
                    sum: r_sum,
                },
            ) => {
                let count = l_count + r_count;
                let min = if l_min < r_min { l_min } else { r_min };
                let max = if l_max > r_max { l_max } else { r_max };
                let sum = l_sum + r_sum;
                let mean = sum / count as f64;

                Self::NonEmpty {
                    count,
                    min,
                    max,
                    sum,
                    mean,
                }
            }
        }
    }
}
