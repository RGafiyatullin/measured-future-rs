pub trait Meter {
    fn enter_poll(&mut self);
    fn leave_poll(&mut self, is_complete: bool);
}
