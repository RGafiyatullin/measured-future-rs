use std::collections::HashMap;

use super::*;

use crate::report_sinks::aggregating::AggregatedReport;

#[derive(Debug, Default, ::serde::Serialize)]
pub struct Aggregator {
    current_stack: Vec<&'static str>,
    pub props: ScopeProps,
    pub sub: HashMap<&'static str, Scope>,
}

#[derive(Debug, Default, ::serde::Serialize)]
pub struct Scope {
    pub props: ScopeProps,
    pub sub: HashMap<&'static str, Scope>,
}

#[derive(Debug, Default, ::serde::Serialize)]
pub struct ScopeProps {
    pub polls: Vec<u128>,
    pub comps: Vec<u128>,
}

impl AggregatedReport<DefaultMeterReport> for Aggregator {
    fn add(&mut self, event: DefaultMeterReport) {
        match event {
            DefaultMeterReport::Enter(key) => {
                self.current_stack.push(key);
            }
            DefaultMeterReport::Leave(key) => {
                let popped = self.current_stack.pop();
                assert_eq!(popped, Some(key));
            }
            DefaultMeterReport::SinglePoll(duration) => {
                self.with_current_scope(|p| p.polls.push(duration.as_nanos()));
            }
            DefaultMeterReport::Completion(duration) => {
                self.with_current_scope(|p| p.comps.push(duration.as_nanos()));
            }
        }
    }
    fn is_complete(&self) -> bool {
        self.current_stack.is_empty()
    }
}

impl Aggregator {
    fn with_current_scope<F, Out>(&mut self, f: F) -> Out
    where
        F: FnOnce(&mut ScopeProps) -> Out,
    {
        let path = &self.current_stack[..];
        let props = &mut self.props;
        let tree = &mut self.sub;

        with_sub_tree(props, tree, path, f)
    }
}

fn with_sub_tree<F, Out>(
    props: &mut ScopeProps,
    tree: &mut HashMap<&'static str, Scope>,
    path: &[&'static str],
    f: F,
) -> Out
where
    F: FnOnce(&mut ScopeProps) -> Out,
{
    if let Some(head) = path.first() {
        let tail = &path[1..];
        if let Some(next) = tree.get_mut(head) {
            let props = &mut next.props;
            let tree = &mut next.sub;
            with_sub_tree(props, tree, tail, f)
        } else {
            let mut props = Default::default();
            let mut sub = Default::default();
            let ret = with_sub_tree(&mut props, &mut sub, tail, f);
            let scope = Scope { props, sub };
            tree.insert(*head, scope);
            ret
        }
    } else {
        f(props)
    }
}
