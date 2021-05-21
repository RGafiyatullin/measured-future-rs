use std::collections::HashMap;
use std::time::Instant;

use crate::report::Report;
use crate::report::Scope;
use crate::report::Stats;

use super::Frame;

#[derive(Debug)]
pub struct Acc {
    roots: HashMap<&'static str, usize>,
    stack: Vec<usize>,
    frames: Vec<Frame>,
}

impl Acc {
    pub fn empty() -> Self {
        Self {
            roots: Default::default(),
            stack: Default::default(),
            frames: Default::default(),
        }
    }

    pub fn flush(&mut self, start: Instant, end: Instant) -> Report {
        let flush_start_at = Instant::now();

        let mut scopes = HashMap::new();

        for (root_k, root_idx) in &self.roots {
            flush_scope(&self.frames, &mut scopes, root_k, *root_idx);
        }

        for frame in &mut self.frames {
            frame.reset();
        }

        let report = Report {
            start,
            end,
            scopes,
            overhead: flush_start_at.elapsed(),
        };

        report
    }

    pub fn push(&mut self, key: &'static str) {
        use std::collections::hash_map::Entry;

        let next_frame_idx = self.frames.len();

        if let Some(current_frame_idx) = self.stack.last().copied() {
            // 1. get current frame
            let frame = &mut self.frames[current_frame_idx];

            // 2. get or create its child
            let (child_frame_idx, should_allocate) = match frame.children.entry(key) {
                Entry::Vacant(vacant) => {
                    let _ = vacant.insert(next_frame_idx);
                    (next_frame_idx, true)
                }
                Entry::Occupied(occupied) => (*occupied.get(), false),
            };

            if should_allocate {
                self.frames.push(Frame::empty());
            }

            // 3. push the child's index onto the stack
            self.stack.push(child_frame_idx);
        } else {
            // 1. get or create root
            let (root_idx, should_allocate) = match self.roots.entry(key) {
                Entry::Occupied(occupied) => (*occupied.get(), false),
                Entry::Vacant(vacant) => {
                    vacant.insert(next_frame_idx);
                    (next_frame_idx, true)
                }
            };

            if should_allocate {
                self.frames.push(Frame::empty());
            }

            // 2. push the root's index onto the stack
            self.stack.push(root_idx);
        }
    }

    pub fn pop(&mut self) -> &mut Frame {
        let popped_frame_idx = self
            .stack
            .pop()
            .expect("Attempt to pop a frame from empty stack");
        &mut self.frames[popped_frame_idx]
    }
}

fn flush_scope(
    frames: &[Frame],
    scopes: &mut HashMap<&'static str, Scope>,
    key: &'static str,
    idx: usize,
) {
    use std::collections::hash_map::Entry;

    let scope =
        match scopes.entry(key) {
            Entry::Vacant(vacant) => vacant.insert(Default::default()),
            Entry::Occupied(_occupied) => unreachable!("We do not expect duplicate scope-names since they are provided by the HashMap iterator"),
        };

    let mut sub_scopes = HashMap::new();

    let frame = &frames[idx];

    for (sub_key, sub_idx) in &frame.children {
        flush_scope(frames, &mut sub_scopes, sub_key, *sub_idx);
    }

    scope.poll = Stats::from(&frame.poll_durations[..]);
    scope.comp = Stats::from(&frame.completion_durations[..]);
    scope.scopes = sub_scopes;
}
