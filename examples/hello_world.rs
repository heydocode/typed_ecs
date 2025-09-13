//! This example demonstrates the simplest possible plugin in `typed_ecs`:
//! - A plugin without any fields (unit-like struct)
//! - Prints "Hello, World!", and exits
//!
//! Key Concepts:
//! - `App` and the builder pattern (`App::new().add_plugin::<Plugin>().build().run()`)
//! - Implementing the `Plugin` trait with lifecycle hooks (`build`, `startup`, `update_exit_status`)
//! - Gracefully requesting the app to stop
//!
//! Expected Output:
//! ```text
//! Hello, World!
//! ```

use typed_ecs::{app::App, plugin::Plugin, shared_data::SharedData};

/// A unit-like plugin that prints "Hello, World!" once and exits.
struct HelloWorldPlugin;

impl<SD: SharedData> Plugin<SD> for HelloWorldPlugin {
    /// Defines how the plugin is constructed when the ECS starts.
    fn build() -> Self {
        Self
    }

    /// Runs once after all plugins have been built.
    fn startup(&mut self) {
        println!("Hello, World!");
    }

    /// Checks whether the ECS loop should continue or stop.
    fn update_exit_status(&self, should_exit: &mut typed_ecs::app::ShouldExit) {
        // Always request exit after the first loop iteration
        should_exit.request_exit();
    }
}

fn main() {
    App::new()
        .add_plugin::<HelloWorldPlugin>() // Register the plugin
        .build() // Finalize app configuration
        .run();  // Run the ECS loop (startup -> update -> exit check)
}
