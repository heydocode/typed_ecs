use std::{thread::sleep, time::Duration};

use typed_ecs::{
    app::App,
    macros::generate_collection,
    plugin::Plugin,
    shared_data::{PhantomSharedData, SharedData}, should_exit::ShouldExit,
};

struct OnExitPlugin;

impl<SD: SharedData> Plugin<SD> for OnExitPlugin {
    fn build() -> Self {
        Self
    }

    fn exit_check<S: ShouldExit>(&mut self, should_exit: &mut S, _sd: &SD) {
        should_exit.request_exit();
    }
    fn on_exit(&mut self, _sd: &SD) {
        println!(
            "\non_exit hook started execution!\nWaiting 3 seconds | simulating computation-heavy on_exit (even if it shouldn't be)..."
        );
        sleep(Duration::from_secs(3));
        println!("\non_exit hook successfully executed!\n");
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "profile")]
    typed_ecs::profile::setup_default_profiling();

    generate_collection!(OnExitPlugin);
    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();
    App::new(collection).run().await;
}
