use core::marker::PhantomData;
use typed_ecs::macros::generate_collection;
use typed_ecs::{
    app::App,
    app::ShouldExit,
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
    fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
        self,
        app: App<SD, PC, Executor>,
    ) {
        let mut plugin_collection = app.plugin_collection;
        let mut sd = app.shared_data;
        // let mut should_exit = app.should_exit;

        println!(
            "The App is being runned by a custom executor. It executes each schedule 100 times, then exits, while ignoring exit checks. On-exit hooks are preserved."
        );

        plugin_collection.startup_ref_sd_all(&sd);
        plugin_collection.startup_mutref_sd_all(&mut sd);

        // while !should_exit.get_val() {
        // Replace by custom: run 100 iterations then exit
        for iter in 1..=100 {
            if iter == 1 {
                println!("Beginning iter {}...", iter);
                println!("Here, each `.` is a new iteration:");
            } else if iter != 100 {
                print!(".");
            } else {
                println!("\nBeginning iter {}...", iter);
            }
            plugin_collection.pre_update_ref_sd_all(&sd);
            plugin_collection.pre_update_mutref_sd_all(&mut sd);
            plugin_collection.update_ref_sd_all(&sd);
            plugin_collection.update_mutref_sd_all(&mut sd);
            plugin_collection.post_update_ref_sd_all(&sd);
            plugin_collection.post_update_mutref_sd_all(&mut sd);
            // Exit only after 100 iterations, don't run the exit check
            // plugin_collection.exit_check_all(&mut should_exit, &sd);
        }
        println!("Calling on-exit hook...");
        plugin_collection.on_exit_all(&sd);
    }
}

fn main() {
    println!("Beginning of the `main` function...");
    generate_collection!(EmptyPlugin);
    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();
    App::new_with_executor(collection, PhantomData::<CustomExecutor>).run();
    println!("Ending of the `main` function...");
}
