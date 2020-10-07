
mod frame;
pub use frame::Frame;

mod state;
pub use state::State;

mod report;
pub use report::Report;

use std::cell::RefCell;

thread_local!(pub static STORAGE: RefCell<State> = RefCell::new(State::new()));