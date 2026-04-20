use core::marker::PhantomData;

#[cfg(feature = "profile")]
use tracing::trace;

use crate::executor::{DefaultExecutor, ExecutorTrait};

use crate::{plugin_collection::PluginCollection, shared_data::SharedData};

pub struct App<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait = DefaultExecutor>
{
    pub executor: PhantomData<Executor>,
    pub shared_data: SD,
    pub plugin_collection: PC,
}

impl<SD: SharedData, PC: PluginCollection<SD>> App<SD, PC, DefaultExecutor> {
    pub fn new(plugin_collection: PC) -> App<SD, PC, DefaultExecutor> {
        App::<SD, PC, DefaultExecutor> {
            executor: PhantomData::<DefaultExecutor>,
            shared_data: SD::build(),
            plugin_collection,
        }
    }
}

impl<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait> App<SD, PC, Executor> {
    pub fn new_with_executor(plugin_collection: PC, executor: PhantomData<Executor>) -> Self {
        Self {
            executor,
            shared_data: SD::build(),
            plugin_collection,
        }
    }

    pub async fn run(&mut self) {
        #[cfg(feature = "profile")]
        let _guard = tracing::info_span!("Executor Runtime").entered();
        let mut executor = Executor::init();
        executor.run(self).await;
    }
}

impl<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait> Drop
    for App<SD, PC, Executor>
{
    fn drop(&mut self) {
        #[cfg(feature = "profile")]
        let _guard = tracing::info_span!("Executor OnExit hooks").entered();
        self.plugin_collection.on_exit_all(&self.shared_data);
    }
}
