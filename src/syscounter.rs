pub trait SysCounter {
    fn new() -> Self;
    fn init(&self);
    fn start_counter(&self, counter: usize);
    fn stop_counter(&self, counter: usize);
    fn reset_counter(&self, counter: usize);
}