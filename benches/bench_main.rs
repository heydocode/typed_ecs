use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    // If profile feature is disabled, this does nothing
    benchmarks::profile::setup,
    benchmarks::fuzz_increment_plugins::benches,
    benchmarks::fuzz_empty_plugins::benches,
}