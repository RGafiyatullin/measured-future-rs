pub trait AggregatedReport<E> {
    fn add(&mut self, event: E);
    fn is_complete(&self) -> bool;
}
