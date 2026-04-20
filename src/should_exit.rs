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
