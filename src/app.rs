use std::marker::PhantomData;

use crate::{
    plugin::Plugin,
    pluginlist::{AllocatedPluginList, PluginList},
    shared_data::{PhantomSharedData, SharedData},
};

/// ECS holder field, that is distributed for all plugins
/// that want to be able to gracefully shutdown the ECS.
/// As soon as its value is true, all plugins have their
/// `on_exit` method called, followed by loop breaking and
/// then app shutdown.
pub struct ShouldExit(bool);

impl ShouldExit {
    /// Changes ShouldExit struct value to true,
    /// causing a graceful shutdown.
    pub fn request_exit(&mut self) {
        self.0 = true;
    }

    pub fn build() -> Self {
        Self(false)
    }
}

pub struct App;

impl App {
    pub fn new() -> StaticApp<PhantomSharedData, ()> {
        StaticApp::<PhantomSharedData, ()> {
            plugins: PhantomData,
            shared_data: PhantomData,
            should_exit: PhantomData,
        }
    }

    pub fn new_with_world<SD: SharedData>() -> StaticApp<SD, ()> {
        StaticApp::<SD, ()> {
            plugins: PhantomData,
            shared_data: PhantomData,
            should_exit: PhantomData,
        }
    }
}

pub struct StaticApp<SD: SharedData, PL: PluginList<SD>> {
    plugins: PhantomData<PL>,
    shared_data: PhantomData<SD>,
    should_exit: PhantomData<ShouldExit>,
}

impl<SD: SharedData, PL: PluginList<SD>> StaticApp<SD, PL> {
    pub fn add_plugin<P: Plugin<SD>>(&self) -> StaticApp<SD, (P, PL)> {
        StaticApp::<SD, (P, PL)> {
            plugins: PhantomData,
            shared_data: PhantomData::<SD>,
            should_exit: PhantomData::<ShouldExit>,
        }
    }

    pub fn build(&self) -> RuntimeApp<SD, PL> where PL: AllocatedPluginList<SD> {
        RuntimeApp::<SD, PL> {
            plugins: PL::build_all(),
            shared_data: SD::build(),
            should_exit: ShouldExit::build(),
        }
    }
}

pub struct RuntimeApp<SD: SharedData, PL: PluginList<SD> + AllocatedPluginList<SD>> {
    plugins: PL,
    shared_data: SD,
    should_exit: ShouldExit,
}

impl<SD: SharedData, PL: PluginList<SD> + AllocatedPluginList<SD>> RuntimeApp<SD, PL> {
    pub fn run(mut self) {
        let plugins = &mut self.plugins;
        let sd = &mut self.shared_data;
        let should_exit = &mut self.should_exit;

        plugins.startup_all();

        while !&should_exit.0 {
            plugins.pre_update_all();
            plugins.update_all();
            plugins.post_update_all();

            plugins.access_ref_sd_all(&sd);
            plugins.access_mutref_sd_all(sd);

            plugins.update_exit_status_all(should_exit);
            plugins.update_exit_status_with_sd_all(should_exit, &sd);
        }
        plugins.on_exit_all();
    }
}