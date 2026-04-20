#![allow(async_fn_in_trait)]

#[cfg(feature = "profile")]
use tracing::trace;

use crate::{app::App, plugin_collection::PluginCollection, shared_data::SharedData, should_exit::ShouldExit};

pub struct DefaultExecutor;

impl ExecutorTrait for DefaultExecutor {
    fn init() -> Self {
        Self
    }
    async fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        &mut self,
        app: &mut App<SD, PC, Executor>,
    ) {
        let mut should_exit = false;

        #[cfg(all(feature = "parallel-global-pool", debug_assertions))]
        if rayon::ThreadPoolBuilder::new()
            // .num_threads(PC::PLUGIN_NUM)
            .thread_name(|i| std::format!("ecs-worker-{i}"))
            .build_global()
            .is_err()
        {
            std::eprintln!("Unable to create a global thread pool. One already exists ?");
        }

        // If not in build with dbg assertions, silently fail
        #[cfg(all(feature = "parallel-global-pool", not(debug_assertions)))]
        let _ = rayon::ThreadPoolBuilder::new()
            // .num_threads(PC::PLUGIN_NUM)
            .thread_name(|i| std::format!("ecs-worker-{i}"))
            .build_global();

        app.plugin_collection.startup_all(&app.shared_data);
        app.plugin_collection
            .apply_startup_all(&mut app.shared_data);

        app.plugin_collection
            .async_startup_all(&app.shared_data)
            .await;
        app.plugin_collection
            .apply_async_startup_all(&mut app.shared_data);

        loop {
            app.plugin_collection.pre_update_all(&app.shared_data);
            app.plugin_collection
                .apply_pre_update_all(&mut app.shared_data);

            app.plugin_collection.update_all(&app.shared_data);
            app.plugin_collection.apply_update_all(&mut app.shared_data);

            app.plugin_collection.post_update_all(&app.shared_data);
            app.plugin_collection
                .apply_post_update_all(&mut app.shared_data);

            app.plugin_collection
                .exit_check_all(&mut should_exit, &app.shared_data);

            if should_exit.is_true() {
                break;
            }

            app.plugin_collection
                .async_update_all(&app.shared_data)
                .await;
            app.plugin_collection
                .apply_async_update_all(&mut app.shared_data);
        }
    }

    fn run_exit_hooks<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        app: &mut App<SD, PC, Executor>,
    ) {
        app.plugin_collection.on_exit_all(&app.shared_data);
    }
}

pub trait ExecutorTrait {
    fn init() -> Self;
    async fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        &mut self,
        app: &mut App<SD, PC, Executor>,
    );
    fn run_exit_hooks<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        app: &mut App<SD, PC, Executor>,
    );
}
