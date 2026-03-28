#![allow(async_fn_in_trait)]
use crate::{app::App, plugin_collection::PluginCollection, shared_data::SharedData};

pub struct DefaultExecutor;

impl ExecutorTrait for DefaultExecutor {
    fn init() -> Self {
        Self
    }
    async fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        &mut self,
        app: &mut App<SD, PC, Executor>,
    ) {
        app.plugin_collection.startup_all(&app.shared_data);
        app.plugin_collection.apply_startup_all(&mut app.shared_data);
        
        app.plugin_collection.async_startup_all(&app.shared_data).await;
        app.plugin_collection.apply_async_startup_all(&mut app.shared_data);

        loop {
            app.plugin_collection.pre_update_all(&app.shared_data);
            app.plugin_collection.apply_pre_update_all(&mut app.shared_data);
            
            app.plugin_collection.update_all(&app.shared_data);
            app.plugin_collection.apply_update_all(&mut app.shared_data);
            
            app.plugin_collection.post_update_all(&app.shared_data);
            app.plugin_collection.apply_post_update_all(&mut app.shared_data);
            
            app.plugin_collection.exit_check_all(&mut app.should_exit, &app.shared_data);
            
            if app.should_exit.get_val() {
                break;
            }
            
            app.plugin_collection.async_update_all(&app.shared_data).await;
            app.plugin_collection.apply_async_update_all(&mut app.shared_data);
        }
        
        app.plugin_collection.on_exit_all(&app.shared_data);
        app.plugin_collection.async_on_exit_all(&app.shared_data).await;
    }
}

pub trait ExecutorTrait {
    fn init() -> Self;
    async fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        &mut self,
        app: &mut App<SD, PC, Executor>,
    );
}
