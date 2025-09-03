use crate::{
    pluginlist::{AllocatedPluginList, PluginList, UnallocatedPluginList},
    shared_data::{PhantomSharedData, SharedData},
};

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

pub struct App<SD: SharedData, PL: PluginList<SD>, const ALLOCATED: bool> {
    plugins: PL,
    shared_data: SD,
    should_exit: ShouldExit,
}

impl<SD: SharedData, PL: PluginList<SD> + UnallocatedPluginList<SD>> App<SD, PL, false> {
    pub fn new(plugins: PL) -> Self {
        Self {
            plugins: plugins,
            shared_data: SD::build(),
            should_exit: ShouldExit(false),
        }
    }

    fn build() -> App<SD, <PL as UnallocatedPluginList<SD>>::Allocated, true> {
        let plugins = PL::build();
        App {
            plugins: plugins,
            shared_data: SD::build(),
            should_exit: ShouldExit(false),
        }
    }

    pub fn run(&mut self) {
        let mut app = Self::build();

        let mut plugins = app.plugins;
        plugins.startup_all();

        while !app.should_exit.0 {
            plugins.pre_update_all();
            plugins.update_all();
            plugins.post_update_all();
        }
    }
}
