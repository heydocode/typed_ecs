/// Allows for using an u8 instead of bool,
/// to not loose 7 bits because of alignment.
/// Moreover introduces access safety.
/// E.g. here we can only toggle on, not off.
pub trait ShouldExit {
    fn request_exit(&mut self);
    fn is_true(&self) -> bool;
}

impl ShouldExit for bool {
    fn request_exit(&mut self) {
        *self = true;
    }
    
    fn is_true(&self) -> bool {
        *self
    }
}
