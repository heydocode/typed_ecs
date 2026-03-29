use core::marker::PhantomData;
use typed_ecs::macros::generate_collection;
use typed_ecs::{
    app::App,
    executor::ExecutorTrait,
    plugin::Plugin,
    plugin_collection::PluginCollection,
    shared_data::{PhantomSharedData, SharedData},
};

struct EmptyPlugin;

impl<SD: SharedData> Plugin<SD> for EmptyPlugin {
    fn build() -> Self {
        Self
    }
}

struct CustomExecutor;

impl ExecutorTrait for CustomExecutor {
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

#[tokio::main]
async fn main() {
    println!("Beginning of the `main` function...");
    generate_collection!(EmptyPlugin);
    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();
    App::new_with_executor(collection, PhantomData::<CustomExecutor>).run().await;
    println!("Ending of the `main` function...");
}
