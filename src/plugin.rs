use core::marker::PhantomData;

use crate::{
    app::ShouldExit,
    shared_data::{PhantomSharedData, SharedData},
};

pub trait Plugin<SD: SharedData = PhantomSharedData> {
    /// Return an instance of the Plugin
    fn build() -> Self;
    /// As soon as the App built instance is runned
    /// (`app.run()`), this method will be executed.
    /// Just after that all plugins finished this method,
    /// the update loop begins. Note that `pre_update` method is
    /// called after all plugins finish executing this method.
    /// This blank implementation will be optimized away by the compiler if
    /// the plugin doesn't reimplement this method.
    #[inline(always)]
    fn startup(&mut self) {}
    /// Schedule, this method runs each loop execution. It gets runned after
    /// the execution of `update_exit_status_with_sd` of previous loop execution.
    /// This blank implementation will be optimized away by the compiler if
    /// the plugin doesn't reimplement this method.
    #[inline(always)]
    fn pre_update(&mut self) {}
    /// Schedule, gets runned after the `pre_update` method.
    /// This blank implementation will be optimized away by the compiler if
    /// the plugin doesn't reimplement this method.
    #[inline(always)]
    fn update(&mut self) {}
    /// Schedule, gets runned after the `update` method, and is
    /// followed by `access_ref_sd`.
    /// This blank implementation will be optimized away by the compiler if
    /// the plugin doesn't reimplement this method.
    #[inline(always)]
    fn post_update(&mut self) {}
    /// Any plugin can access SharedData instance by reference
    /// once a frame.
    /// Plugins can for instance update their own data based on
    /// the accessed data, or execute instant logic with it.
    /// This blank implementation will be optimized away by the compiler if
    /// the plugin doesn't reimplement this method.
    /// Schedule, runs after `post_update`.
    #[inline(always)]
    fn access_ref_sd(&mut self, _sd: &SD) {}
    /// Similar to the `access_ref_sd` method, but provides
    /// a mutable reference to the SharedData instance.
    /// This blank implementation will be optimized away by the compiler if
    /// the plugin doesn't reimplement this method.
    /// Schedule, runs after `access_ref_sd`.
    #[inline(always)]
    fn access_mutref_sd(&mut self, _sd: &mut SD) {}
    /// This blank implementation will be optimized away by the compiler if
    /// the plugin doesn't reimplement this method.
    /// As soon as this method changes should_exit value to true, the ECS calls
    /// `on_exit` methods just before exiting the loop.
    ///
    /// Here's what's needed to change ShouldExit struct value to true:
    /// ```rust
    /// should_exit.request_exit();
    /// ```
    ///
    /// Note that as soon as exit is requested, it can't be cancelled in any way!
    /// Schedule, runs after `access_mutref_sd`.
    #[inline(always)]
    fn update_exit_status(&self, _should_exit: &mut ShouldExit) {}
    /// Same as for the `update_exit_status` method, but the plugin can access SharedData
    /// too, so it can operate according to more informations.
    /// Schedule, runs after `update_exit_status`.
    #[inline(always)]
    fn update_exit_status_with_sd(&self, _should_exit: &mut ShouldExit, _sd: &SD) {}
    /// This method runs after `update_exit_status_with_sd` schedule if
    /// a plugin has requested shutdown.
    #[inline(always)]
    fn on_exit(&mut self) {}
}

impl<SD: SharedData> Plugin<SD> for () {
    fn build() -> Self {
        ()
    }
}

impl<SD: SharedData> Plugin<SD> for PhantomData<()> {
    fn build() -> Self {
        PhantomData
    }
}