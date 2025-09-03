use std::marker::PhantomData;

use typed_ecs::{app::App, plugin::Plugin, pluginlist::UnallocatedPluginList};

fn main() {
    struct Example(u64);
    impl Plugin for Example {
        fn build() -> Self {
            Self(0)
        }

        fn update(&mut self) {
            self.0 += 1;
            println!("Counter: {}", self.0);
        }
    }
    let mut app = App::new(PhantomData::<()>.add_plugin(Example(0)));
    app.run();
}
