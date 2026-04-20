use rand::random_range;
use seq_macro::seq;
use typed_ecs::should_exit::ShouldExit;
use std::{thread::sleep, time::Duration};
use typed_ecs::macros::generate_collection;
use typed_ecs::shared_data::PhantomSharedData;
use typed_ecs::{app::App, plugin::Plugin, shared_data::SharedData};

struct ExitCounterPlugin;

impl<SD: SharedData> Plugin<SD> for ExitCounterPlugin {
    fn build() -> Self {
        Self
    }

    fn exit_check<S: ShouldExit>(&mut self, should_exit: &mut S, _sd: &SD) {
        should_exit.request_exit();
    }
}

seq!(N in 1..=250 {
    struct Plugin~N;

    impl<SD: SharedData> Plugin<SD> for Plugin~N {
        fn build() -> Self {
            Self
        }
        fn update(&mut self, _sd: &SD) {
            sleep(Duration::from_millis(random_range(0..300)));
        }
    }
});

#[tokio::main]
async fn main() {
    #[cfg(feature = "profile")]
    typed_ecs::profile::setup_default_profiling();

    seq!(N in 1..=250 {
        generate_collection!(#(Plugin~N,)* ExitCounterPlugin);
    });

    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();

    App::new(collection).run().await;
}
