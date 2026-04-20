use criterion::{Criterion, criterion_group};
use seq_macro::seq;
use typed_ecs::macros::generate_collection;
use typed_ecs::should_exit::ShouldExit;
use typed_ecs::{app::App, plugin::Plugin, shared_data::SharedData};

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

    fn exit_check<S: ShouldExit>(&mut self, should_exit: &mut S, sd: &SD) {
        if sd.get_i() == 2 {
            should_exit.request_exit();
        }
    }

    fn apply_update(&mut self, sd: &mut SD) {
        sd.increment_i();
    }
}

seq!(N in 1..=100 {
    struct Plugin~N;
    impl <SD: SharedData>Plugin<SD> for Plugin~N {
        fn build() -> Self {
            Self
        }
    }
});

pub fn run_fuzzed_plugins(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();
    let mut group = c.benchmark_group("100 empty plugins");

    seq!(N in 1..=100 {
        generate_collection!(#(Plugin~N,)* ExitCounterPlugin);
    });

    group.bench_function("fuzz_empty_plugins", move |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let collection: GeneratedPluginCollection<SDimpl> = build_generated_collection();

                App::new(collection).run().await;

                std::hint::black_box(())
            });
    });

    group.finish();
}

criterion_group!(benches, run_fuzzed_plugins);
