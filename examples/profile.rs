use std::{thread::sleep, time::Duration};

use typed_ecs::{
    app::{App, ShouldExit},
    macros::generate_collection,
    plugin::Plugin,
    plugin_collection::PluginCollection,
    profile::setup_default_profiling,
    shared_data::SharedData,
};

struct Sleep200msPlugin;

impl<SD: SharedData> Plugin<SD> for Sleep200msPlugin {
    fn build() -> Self {
        Self
    }
    fn update(&mut self, _sd: &SD) {
        // Makes the plugin's system noticeable
        // in the produced profiling trace.
        sleep(Duration::from_millis(200));
    }
}

struct Plugin1;
impl<SD: SharedData> Plugin<SD> for Plugin1 {
    fn build() -> Self {
        Self
    }
    fn startup(&mut self, _sd: &SD) {
        println!("Hello from plugin 1!");
    }
}

struct Plugin3;
impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for Plugin3 {
    fn build() -> Self {
        Self
    }
    fn startup(&mut self, sd: &SD) {
        println!("Initial val value: {}", sd.get_val());
    }

    fn apply_pre_update(&mut self, sd: &mut SD) {
        let val = sd.get_val();

        if !(val >= u8::MAX - 1) {
            sd.set_val(val + 1);
        } else {
            sd.set_val(0);
            sd.incr_i();
        }
    }
}

struct Plugin2;

impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for Plugin2 {
    fn build() -> Self {
        Self
    }
    fn post_update(&mut self, sd: &SD) {
        let val = sd.get_val();

        println!("Current val: {}", val);
    }

    fn on_exit(&mut self, sd: &SD) {
        let val = sd.get_val();
        let i = sd.get_i();

        println!("Current val: {}", val);
        println!("Iterations number: {}", i);
    }
}

struct CtrlCHandler;

impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for CtrlCHandler {
    fn build() -> Self {
        Self
    }
    fn exit_check(&mut self, should_exit: &mut ShouldExit, sd: &SD) {
        if sd.get_i() >= 100 {
            should_exit.request_exit();
        }
    }
}

trait AdditionalRequirement {
    fn get_val(&self) -> u8;
    fn set_val(&mut self, val: u8);
    fn get_i(&self) -> u64;
    fn incr_i(&mut self);
}

struct SDimpl {
    val: u8,
    i: u64,
}

impl SharedData for SDimpl {
    fn build() -> Self {
        Self { val: 0, i: 0 }
    }
}

impl AdditionalRequirement for SDimpl {
    fn get_val(&self) -> u8 {
        self.val
    }
    fn set_val(&mut self, val: u8) {
        self.val = val;
    }
    fn get_i(&self) -> u64 {
        self.i
    }
    fn incr_i(&mut self) {
        self.i += 1;
    }
}

#[tokio::main]
async fn main() {
    setup_default_profiling();

    generate_collection!(CtrlCHandler, Plugin1, Plugin2, Sleep200msPlugin, Plugin3,);
    // This is indeed a constant! A ZST, assembling multiple plugins into one scheduled runtime.
    let collection: GeneratedPluginCollection<SDimpl> = build_generated_collection::<SDimpl>();
    App::new(collection).run().await;
}
