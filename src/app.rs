use crate::{
    plugin_collection::PluginCollection,
    shared_data::SharedData
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
}

pub struct App<SD: SharedData, PC: PluginCollection<SD>> {
    shared_data: SD,
    plugin_collection: PC,
    should_exit: ShouldExit,
}

impl<SD: SharedData, PC: PluginCollection<SD>> App<SD, PC> {
    pub fn new(plugin_collection: PC) -> Self {
        Self {
            shared_data: SD::build(),
            plugin_collection,
            should_exit: ShouldExit(false),
        }
    }

    pub fn run(self) {
        let plugin_collection = &self.plugin_collection;
        let mut sd = self.shared_data;
        let mut should_exit = self.should_exit;

        plugin_collection.startup_ref_sd_all(&sd);
        plugin_collection.startup_mutref_sd_all(&mut sd);

        while !&should_exit.0 {
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