use typed_ecs::{
    app::{App, ShouldExit},
    macros::generate_collection,
    plugin::Plugin,
    shared_data::{PhantomSharedData, SharedData},
};

struct HelloWorldPlugin;

impl<SD: SharedData> Plugin<SD> for HelloWorldPlugin {
    fn build() -> Self {
        Self
    }
    fn startup(&mut self, _sd: &SD) {
        println!("Hello, World!");
    }
    // This system runs in the end of each cycle,
    // so after startup, and after all updates.
    fn exit_check(&mut self, should_exit: &mut ShouldExit, _sd: &SD) {
        // Requests the program to exit. This request is guaranteed
        // to be heard, but not guaranteed to make the app exit immediately
        // (because it needs to run on_exit hooks).
        should_exit.request_exit();
    }
}

#[tokio::main]
async fn main() {
    generate_collection!(HelloWorldPlugin);
    // PhantomSharedData indicates that no plugin require any memory space to operate.
    // See more about it in the SharedData trait (src/app.rs)
    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();
    App::new(collection).run().await;
}
