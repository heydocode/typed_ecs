use seq_macro::seq;
use std::time::Duration;
use typed_ecs::macros::generate_collection;
use typed_ecs::{
    app::{App},
    plugin::Plugin,
    shared_data::SharedData,
};

trait CounterMemory {
    fn get_i(&self) -> u128;
    fn increment_i(&mut self);
}

struct SDimpl {
    i: u128,
}

impl SharedData for SDimpl {
    fn build() -> Self {
        Self { i: 0 }
    }
}

impl CounterMemory for SDimpl {
    fn get_i(&self) -> u128 {
        self.i
    }
    fn increment_i(&mut self) {
        self.i += 1;
    }
}

struct ExitCounterPlugin;

impl<SD: SharedData + CounterMemory> Plugin<SD> for ExitCounterPlugin {
    fn build() -> Self {
        Self
    }

    fn exit_check(&mut self, should_exit: &mut bool, sd: &SD) {
        if sd.get_i() == 2 {
            *should_exit = true;
        }
    }

    fn apply_update(&mut self, sd: &mut SD) {
        sd.increment_i();
    }
}

seq!(N in 1..=50 {
    struct Plugin~N;

    impl<SD: SharedData + CounterMemory> Plugin<SD> for Plugin~N {
        fn build() -> Self {
            Self
        }
        async fn async_update(&mut self, _sd: &SD) {
            tokio::time::sleep(Duration::from_secs(1)).await;
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

    let collection: GeneratedPluginCollection<SDimpl> = build_generated_collection();

    App::new(collection).run().await;
}
