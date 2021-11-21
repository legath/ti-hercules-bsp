pub trait SysCounter {
    fn new(debug_run: bool) -> Self;
    fn init(&self, debug_run: bool);
    fn start_counter(&self, counter: usize);
    fn stop_counter(&self, counter: usize);
    fn reset_counter(&self, counter: usize);
    fn set_period(&self, compare: usize, period: u32);
}