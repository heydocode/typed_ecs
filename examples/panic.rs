use std::{hint::black_box, thread::sleep, time::Duration};

use typed_ecs::{
    app::App,
    macros::generate_collection,
    plugin::Plugin,
    shared_data::{PhantomSharedData, SharedData},
};

struct PanicPlugin;

impl<SD: SharedData> Plugin<SD> for PanicPlugin {
    fn build() -> Self {
        Self
    }

    fn post_update(&mut self, _sd: &SD) {
            panic!("Panicking...");
    }

    fn on_exit(&mut self, _sd: &SD) {
        println!(
            "\non_exit hook started execution!\nWaiting 3 seconds | simulating computation-heavy on_exit (even if it shouldn't be)..."
        );
        sleep(Duration::from_secs(3));
        println!("\non_exit hook successfully executed after panic!\n");
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "profile")]
    typed_ecs::profile::setup_default_profiling();
    
    generate_collection!(PanicPlugin);
    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();
    App::new(collection).run().await;
}
