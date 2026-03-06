use criterion::{Criterion, criterion_group};
use macros::generate_collection;
use seq_macro::seq;
use typed_ecs::{app::{App, ShouldExit}, plugin::Plugin, plugin_collection::PluginCollection, shared_data::{SharedData}};

trait CounterMemory {
    fn get_i(&self) -> u128;
    fn increment_i(&mut self);
}

struct SDimpl {
    i: u128
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

seq!(N in 1..=500 {
    struct Plugin~N;
    impl <SD: SharedData + CounterMemory>Plugin<SD> for Plugin~N {
        fn update_mutref_sd(&self, sd: &mut SD) {
            sd.increment_i();
        }
        
        fn exit_check(&self, should_exit: &mut ShouldExit, sd: &SD) {
            if sd.get_i() == 1_000 {
                should_exit.request_exit();
            }
        }
    }
});

pub fn run_fuzzed_plugins(c: &mut Criterion) {
    let mut group = c.benchmark_group("500 increment plugins");
    
    seq!(N in 1..=500 {
        generate_collection!(#(Plugin~N,)*);
    });
    
    const COLLECTION: GeneratedPluginCollection<SDimpl> = build_generated_collection();
    group.bench_function("fuzz_empty_plugins", move |b| {
        b.iter(|| App::new(COLLECTION).run());
    });
    group.finish();
}

criterion_group!(benches, run_fuzzed_plugins);