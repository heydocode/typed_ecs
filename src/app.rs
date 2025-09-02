/// ECS holder field, that is distributed for all plugins
/// that want to be able to gracefully shutdown the ECS.
/// As soon as its value is true, all plugins have their
/// `on_exit` method called, followed by loop breaking and
/// then app shutdown.
pub struct ShouldExit(bool);

impl Default for ShouldExit {
    fn default() -> Self {
        Self(false)
    }
}

impl ShouldExit {
    /// Changes ShouldExit struct value to true, 
    /// causing a graceful shutdown.
    pub fn request_exit(&mut self) {
        self.0 = true;
    }
}