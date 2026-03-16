use crate::{plugin_collection::PluginCollection, shared_data::SharedData, app::App};

#[cfg(feature = "std")]
pub struct DefaultExecutor;

#[cfg(feature = "std")]
impl ExecutorTrait for DefaultExecutor {
    fn init() -> Self {
        Self
    }
    fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        self,
        app: App<SD, PC, Executor>,
    ) {
        tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let plugin_collection = &app.plugin_collection;
                    let mut sd = app.shared_data;
                    let mut should_exit = app.should_exit;
            
                    plugin_collection.async_startup_ref_sd_all(&sd).await;
                    plugin_collection.async_startup_mutref_sd_all(&mut sd).await;
                    
                    plugin_collection.startup_ref_sd_all(&sd);
                    plugin_collection.startup_mutref_sd_all(&mut sd);
            
                    while !should_exit.get_val() {
                        plugin_collection.async_pre_update_ref_sd_all(&sd).await;
                        plugin_collection.async_pre_update_mutref_sd_all(&mut sd).await;
                        plugin_collection.async_update_ref_sd_all(&sd).await;
                        plugin_collection.async_update_mutref_sd_all(&mut sd).await;
                        plugin_collection.async_post_update_ref_sd_all(&sd).await;
                        plugin_collection.async_post_update_mutref_sd_all(&mut sd).await;
                        plugin_collection.async_exit_check_all(&mut should_exit, &sd).await;
                        
                        plugin_collection.pre_update_ref_sd_all(&sd);
                        plugin_collection.pre_update_mutref_sd_all(&mut sd);
                        plugin_collection.update_ref_sd_all(&sd);
                        plugin_collection.update_mutref_sd_all(&mut sd);
                        plugin_collection.post_update_ref_sd_all(&sd);
                        plugin_collection.post_update_mutref_sd_all(&mut sd);
                        plugin_collection.exit_check_all(&mut should_exit, &sd);
                    }
                    plugin_collection.on_exit_all(&sd);
                })
    }
}

#[cfg(feature = "no-std")]
pub struct EmbassyExecutor;

#[cfg(feature = "no-std")]
impl ExecutorTrait for EmbassyExecutor {
    fn init() -> Self {
        Self
    }
    fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        self,
        app: App<SD, PC, Executor>,
    ) {
        tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let plugin_collection = &app.plugin_collection;
                    let mut sd = app.shared_data;
                    let mut should_exit = app.should_exit;
            
                    plugin_collection.startup_ref_sd_all(&sd);
                    plugin_collection.startup_mutref_sd_all(&mut sd);
            
                    while !should_exit.get_val() {
                        plugin_collection.async_pre_update_ref_sd_all(&sd).await;
                        plugin_collection.async_pre_update_mutref_sd_all(&mut sd).await;
                        plugin_collection.async_update_ref_sd_all(&sd).await;
                        plugin_collection.async_update_mutref_sd_all(&mut sd).await;
                        plugin_collection.async_post_update_ref_sd_all(&sd).await;
                        plugin_collection.async_post_update_mutref_sd_all(&mut sd).await;
                        plugin_collection.async_exit_check_all(&mut should_exit, &sd).await;
                        
                        plugin_collection.pre_update_ref_sd_all(&sd);
                        plugin_collection.pre_update_mutref_sd_all(&mut sd);
                        plugin_collection.update_ref_sd_all(&sd);
                        plugin_collection.update_mutref_sd_all(&mut sd);
                        plugin_collection.post_update_ref_sd_all(&sd);
                        plugin_collection.post_update_mutref_sd_all(&mut sd);
                        plugin_collection.exit_check_all(&mut should_exit, &sd);
                    }
                    plugin_collection.on_exit_all(&sd);
                })
    }
}

pub trait ExecutorTrait {
    fn init() -> Self;
    fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        self,
        app: App<SD, PC, Executor>,
    );
}
