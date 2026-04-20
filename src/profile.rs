pub use tracing;

/// Sets up a global default subscriber, chosen with enabled features
/// You must not set another global default subscriber, or else the
/// program can either crash, either ignore the second init.
pub fn setup_default_profiling() {
    #[cfg(feature = "profile-tracy")]
    {
        use tracing_subscriber::layer::SubscriberExt;

        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
        )
        .expect("setup tracy layer");
    }
    #[cfg(feature = "profile-forest")]
    {
        tracing_forest::init();
    }
    #[cfg(all(feature = "profile-forest", feature = "profile-tracy"))]
    compile_error!(
        "You cannot enable two profilers at the same time! Enable only one `profile-...` feature!\nCurrently enabled: `profile-tracy` AND `profile-forest`."
    );
    #[cfg(not(any(feature = "profile-forest", feature = "profile-tracy")))]
    compile_error!(
        "You should enable either feature `profile-forest` (zero-setup, built-in profiler), \neither `profile-tracy` (requires Tracy - an external application) enabled to build this example.\nExample usage: cargo run --example profile --features=profile-forest"
    );
}
