use std::process::exit;

use embassy_time::Timer;
use seq_macro::seq;
use typed_ecs::macros::generate_collection;
#[cfg(feature = "profile")]
use typed_ecs::profile::setup_default_profiling;
use typed_ecs::{
    app::{App, ShouldExit},
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

    fn exit_check(&mut self, should_exit: &mut ShouldExit, sd: &SD) {
        if sd.get_i() == 2 {
            should_exit.request_exit();
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
            Timer::after_secs(1).await;
        }
    }
});

#[embassy_executor::main]
async fn main(s: embassy_executor::Spawner) {
    s.spawn(app_run().unwrap());
}

#[embassy_executor::task]
async fn app_run() {
    #[cfg(feature = "profile")]
    setup_default_profiling();
    
    seq!(N in 1..=50 {
        generate_collection!(#(Plugin~N,)* ExitCounterPlugin);
    });

    let collection: GeneratedPluginCollection<SDimpl> = build_generated_collection();

    App::new(collection).run().await;
    
    println!("App terminated!");
    
    // By default, embassy will continue to execute, waiting for new tasks or sleeping
    exit(0);
}