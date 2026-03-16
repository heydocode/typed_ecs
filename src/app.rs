use core::marker::PhantomData;

use crate::executor::{ExecutorTrait, DefaultExecutor};

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
        Executor::init().run(self);
    }
}