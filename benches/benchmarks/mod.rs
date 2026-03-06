pub mod fuzz_increment_plugins;
pub mod fuzz_empty_plugins;
pub mod profile;

use criterion::Bencher;
use typed_ecs::{app::{App, ExecutorTrait}, plugin_collection::PluginCollection, shared_data::SharedData};

pub struct BenchExecutor;

impl ExecutorTrait for BenchExecutor {
    fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(app: App<SD, PC, Executor>) {
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