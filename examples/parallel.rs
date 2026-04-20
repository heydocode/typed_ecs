use seq_macro::seq;
use typed_ecs::shared_data::PhantomSharedData;
use std::{thread::sleep, time::Duration};
use typed_ecs::macros::generate_collection;
use typed_ecs::{
    app::{App, },
    plugin::Plugin,
    shared_data::SharedData,
};

struct ExitCounterPlugin;

impl<SD: SharedData> Plugin<SD> for ExitCounterPlugin {
    fn build() -> Self {
        Self
    }

    fn exit_check(&mut self, should_exit: &mut bool, sd: &SD) {
        *should_exit = true;
    }
}

seq!(N in 1..=50 {
    struct Plugin~N;

    impl<SD: SharedData> Plugin<SD> for Plugin~N {
        fn build() -> Self {
            Self
        }
        fn update(&mut self, _sd: &SD) {
            sleep(Duration::from_secs(1));
        }
    }
});

#[tokio::main]
async fn main() {
    #[cfg(feature = "profile")]
    typed_ecs::profile::setup_default_profiling();

    seq!(N in 1..=50 {
        generate_collection!(#(Plugin~N,)* ExitCounterPlugin);
    });

    let collection: GeneratedPluginCollection<PhantomSharedData> = build_generated_collection();

    App::new(collection).run().await;
}
