use typed_ecs::{
    app::App,
    macros::generate_collection,
    plugin::Plugin,
    shared_data::{PhantomSharedData, SharedData}, should_exit::ShouldExit,
};

struct HelloWorldPlugin;

impl<SD: SharedData> Plugin<SD> for HelloWorldPlugin {
    fn build() -> Self {
        Self
    }

    fn startup(&mut self, _sd: &SD) {
        println!("Hello, World!");
    }

    fn exit_check<S: ShouldExit>(&mut self, should_exit: &mut S, _sd: &SD) {
        should_exit.request_exit();
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "profile")]
    typed_ecs::profile::setup_default_profiling();

    generate_collection!(HelloWorldPlugin);
    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();
    App::new(collection).run().await;
}
