use typed_ecs::{app::App, plugin::Plugin};

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
        fn update_exit_status(&self, should_exit: &mut typed_ecs::app::ShouldExit) {
            if self.0 > 5_000_000 {
                should_exit.request_exit();
            }
        }
        fn on_exit(&mut self) {
            println!("Graceful shutdown!");
        }
    }
    
    App::new().add_plugin::<Example>().build().run();
}
