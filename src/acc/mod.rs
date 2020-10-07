use std::cell::RefCell;

mod acc;
pub use acc::Acc;

mod frame;
pub use frame::Frame;

thread_local! {
    pub static ACC: RefCell<Option<Acc>> = RefCell::new(None);
}
