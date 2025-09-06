use std::marker::PhantomData;

use crate::{app::ShouldExit, plugin::Plugin, shared_data::SharedData};

pub trait PluginList<SD: SharedData> {
    fn build_all() -> Self;
}

pub trait AllocatedPluginList<SD: SharedData>: PluginList<SD> {
    /// Recursively calls `startup` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn startup_all(&mut self);
    /// Recursively calls `pre_update` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn pre_update_all(&mut self);
    /// Recursively calls `update_all` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn update_all(&mut self);
    /// Recursively calls `post_update` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn post_update_all(&mut self);
    /// Recursively calls `access_ref_sd` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn access_ref_sd_all(&mut self, _sd: &SD);
    /// Recursively calls `access_mutref_sd` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn access_mutref_sd_all(&mut self, _sd: &mut SD);
    /// Recursively calls `update_exit_status` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn update_exit_status_all(&self, _should_exit: &mut ShouldExit);
    /// Recursively calls `update_exit_status_with_sd` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn update_exit_status_with_sd_all(&self, _should_exit: &mut ShouldExit, _sd: &SD);
    /// Recursively calls `on_exit` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    /// This is ONLY executed when ShouldExit is set to true and when the ECS loop ends.
    fn on_exit_all(&mut self);
}

impl<SD: SharedData> PluginList<SD> for PhantomData<()> {
    fn build_all() -> Self {
        PhantomData
    }
}

impl<SD: SharedData> PluginList<SD> for () {
    fn build_all() -> Self {
        ()
    }
}

impl<SD: SharedData, Head: Plugin<SD>, Tail: PluginList<SD>> PluginList<SD>
    for PhantomData<(Head, Tail)>
{
    fn build_all() -> Self {
        PhantomData
    }
}

impl<SD: SharedData, Head: Plugin<SD>, Tail: PluginList<SD>> PluginList<SD> for (Head, Tail) {
    fn build_all() -> Self {
        (Head::build(), Tail::build_all())
    }
}

impl<SD: SharedData> AllocatedPluginList<SD> for () {
    fn startup_all(&mut self) {}

    fn pre_update_all(&mut self) {}

    fn update_all(&mut self) {}

    fn post_update_all(&mut self) {}

    fn access_ref_sd_all(&mut self, _sd: &SD) {}

    fn access_mutref_sd_all(&mut self, _sd: &mut SD) {}

    fn update_exit_status_all(&self, _should_exit: &mut ShouldExit) {}

    fn update_exit_status_with_sd_all(&self, _should_exit: &mut ShouldExit, _sd: &SD) {}

    fn on_exit_all(&mut self) {}
}

impl<SD: SharedData, Head: Plugin<SD>, Tail: PluginList<SD> + AllocatedPluginList<SD>>
    AllocatedPluginList<SD> for (Head, Tail)
{
    fn startup_all(&mut self) {
        self.0.startup();
        self.1.startup_all();
    }

    fn pre_update_all(&mut self) {
        self.0.pre_update();
        self.1.pre_update_all();
    }

    fn update_all(&mut self) {
        self.0.update();
        self.1.update_all();
    }

    fn post_update_all(&mut self) {
        self.0.post_update();
        self.1.post_update_all();
    }

    fn access_ref_sd_all(&mut self, sd: &SD) {
        self.0.access_ref_sd(sd);
        self.1.access_ref_sd_all(sd);
    }

    fn access_mutref_sd_all(&mut self, sd: &mut SD) {
        self.0.access_mutref_sd(sd);
        self.1.access_mutref_sd_all(sd);
    }

    fn update_exit_status_all(&self, should_exit: &mut ShouldExit) {
        self.0.update_exit_status(should_exit);
        self.1.update_exit_status_all(should_exit);
    }

    fn update_exit_status_with_sd_all(&self, should_exit: &mut ShouldExit, sd: &SD) {
        self.0.update_exit_status_with_sd(should_exit, sd);
        self.1.update_exit_status_with_sd_all(should_exit, sd);
    }

    fn on_exit_all(&mut self) {
        self.0.on_exit();
        self.1.on_exit_all();
    }
}
