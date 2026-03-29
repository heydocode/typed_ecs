use criterion::{Criterion, criterion_group};
use seq_macro::seq;
use typed_ecs::macros::generate_collection;
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

seq!(N in 1..=100 {
    struct Plugin~N;
    impl <SD: SharedData + CounterMemory>Plugin<SD> for Plugin~N {
        fn build() -> Self {
            Self
        }

        fn apply_update(&mut self, sd: &mut SD) {
            sd.increment_i();
        }

        fn exit_check(&mut self, should_exit: &mut ShouldExit, sd: &SD) {
            if sd.get_i() == 200 {
                should_exit.request_exit();
            }
        }
    }
});

pub fn run_fuzzed_plugins(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();
    let mut group = c.benchmark_group("100 increment plugins");

    seq!(N in 1..=100 {
        generate_collection!(#(Plugin~N,)*);
    });

    group.bench_function("fuzz_increment_plugins", move |b| {
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
