use core::marker::PhantomData;

use crate::{plugin_collection::PluginCollection, shared_data::SharedData};

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

    pub fn get_val(&self) -> bool {
        self.0
    }
}

pub struct App<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait = DefaultExecutor>
{
    pub executor: PhantomData<Executor>,
    pub shared_data: SD,
    pub plugin_collection: PC,
    pub should_exit: ShouldExit,
}

impl<SD: SharedData, PC: PluginCollection<SD>> App<SD, PC, DefaultExecutor> {
    pub fn new(plugin_collection: PC) -> App<SD, PC, DefaultExecutor> {
        App::<SD, PC, DefaultExecutor> {
            executor: PhantomData::<DefaultExecutor>,
            shared_data: SD::build(),
            plugin_collection,
            should_exit: ShouldExit(false),
        }
    }
}

impl<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait> App<SD, PC, Executor> {
    pub fn new_with_executor(plugin_collection: PC, executor: PhantomData<Executor>) -> Self {
        Self {
            executor,
            shared_data: SD::build(),
            plugin_collection,
            should_exit: ShouldExit(false),
        }
    }

    pub fn run(self) {
        Executor::run(self)
    }
}

pub struct DefaultExecutor;

impl ExecutorTrait for DefaultExecutor {
    fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        app: App<SD, PC, Executor>,
    ) {
        let plugin_collection = &app.plugin_collection;
        let mut sd = app.shared_data;
        let mut should_exit = app.should_exit;

        plugin_collection.startup_ref_sd_all(&sd);
        plugin_collection.startup_mutref_sd_all(&mut sd);

        while !should_exit.get_val() {
            plugin_collection.pre_update_ref_sd_all(&sd);
            plugin_collection.pre_update_mutref_sd_all(&mut sd);
            plugin_collection.update_ref_sd_all(&sd);
            plugin_collection.update_mutref_sd_all(&mut sd);
            plugin_collection.post_update_ref_sd_all(&sd);
            plugin_collection.post_update_mutref_sd_all(&mut sd);
            plugin_collection.exit_check_all(&mut should_exit, &sd);
        }
        plugin_collection.on_exit_all(&sd);
    }
}

pub trait ExecutorTrait {
    fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        app: App<SD, PC, Executor>,
    );
}
