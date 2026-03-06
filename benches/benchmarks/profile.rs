use criterion::Criterion;
#[cfg(feature = "profile")]
use typed_ecs::profile::setup_default_profiling;
use criterion::criterion_group;

#[cfg(feature = "profile")]
pub fn setup_profiler(c: &mut Criterion) {
    let group = c.benchmark_group("Setup profiler");
    setup_default_profiling();
    group.finish();
}

#[cfg(not(feature = "profile"))]
pub fn setup_profiler(_c: &mut Criterion) {}

criterion_group!(setup, setup_profiler);