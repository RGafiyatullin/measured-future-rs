use super::*;

pub struct State {
    frames: Vec<Frame>,
    stack: Vec<usize>,
}

impl State {
    pub fn new() -> Self {
        Self {
            frames: vec![Frame::empty()],
            stack: vec![0],
        }
    }
    // pub fn root(&mut self) -> &mut Frame {
    //     &mut self.frames[0]
    // }
    pub fn push(&mut self, key: &'static str) -> &mut Frame {
        use std::collections::hash_map::Entry;

        let next_frame_idx = self.frames.len();

        let current_idx = self.stack.last().copied().expect("pop: No current frame");

        let current_frame = &mut self.frames[current_idx];
        let (child_idx, new_frame) = match current_frame.children.entry(key) {
            Entry::Occupied(occupied) => (*occupied.get(), false),
            Entry::Vacant(vacant) => {
                vacant.insert(next_frame_idx);
                (next_frame_idx, true)
            }
        };

        if new_frame {
            self.frames.push(Frame::empty());
        }

        self.stack.push(child_idx);

        &mut self.frames[child_idx]
    }
    pub fn pop(&mut self) -> &mut Frame {
        let current_idx = self.stack.pop().expect("pop: No current frame");

        &mut self.frames[current_idx]
    }

    pub fn flush(&mut self) -> Report {
        let report = frame_report(&self.frames, 0);
        for frame in &mut self.frames {
            frame.reset();
        }
        report
    }
}

fn frame_report(frames: &[Frame], frame_idx: usize) -> Report {
    let frame = &frames[frame_idx];

    let time = frame.acc;
    let polls = frame.polls;

    let children = frame
        .children
        .iter()
        .map(|(ch_name, ch_idx)| {
            let ch_report = frame_report(frames, *ch_idx);
            (*ch_name, ch_report)
        })
        .collect();

    Report {
        time,
        polls,
        children,
    }
}
