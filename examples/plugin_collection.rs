use typed_ecs::{app::{App, ShouldExit}, macros::generate_collection, plugin::Plugin, plugin_collection::PluginCollection, shared_data::SharedData  };

struct Plugin1;
impl<SD: SharedData> Plugin<SD> for Plugin1 {
    fn startup_ref_sd(&self, _sd: &SD) {
        println!("Hello from plugin 1!");
    }
}

struct Plugin2;
impl<SD: SharedData> Plugin<SD> for Plugin2 { 
    /* Empty plugin, will it be optimized away? For sure! */ 
    
}
impl Plugin2 {
    const MARKER_STRING: &'static str = "This unused plugin hasn't been erazed?";
}

struct Plugin3;
impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for Plugin3 {
    fn startup_ref_sd(&self, sd: &SD) {
        println!("Initial val value: {}", sd.get_val());
    }
    
    fn pre_update_mutref_sd(&self, sd: &mut SD) {
        let val = sd.get_val();
        
        if !(val >= u8::MAX - 1) {
            sd.set_val(val + 1);
        }
        else {
            sd.set_val(0);
            sd.incr_i();
        }
    }
}

struct Plugin4;

impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for Plugin4 {
    fn post_update_ref_sd(&self, sd: &SD) {
        let val = sd.get_val();
        
        println!("Current val: {}", val);
    }
    
    fn on_exit(&self, sd: &SD) {
        let val = sd.get_val();
        let i = sd.get_i();
        
        println!("Current val: {}", val);
        println!("Iterations number: {}", i);
    }
}

struct CtrlCHandler;

impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for CtrlCHandler {
    fn update_exit_status_with_sd(&self, should_exit: &mut ShouldExit, _sd: &SD) {
        if ctrlc_tiny::is_ctrlc_received() {
            should_exit.request_exit();
        }
    }
    
    fn startup_ref_sd(&self, _sd: &SD) {
        ctrlc_tiny::init_ctrlc().unwrap();
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
    i: u64
}

impl SharedData for SDimpl {
    fn build() -> Self {
        Self {
            val: 0,
            i: 0
        }
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

fn main() {
    generate_collection!(
        CtrlCHandler,
        Plugin1, 
        Plugin2, 
        Plugin3,
        Plugin4
    );
    // This is indeed a constant! A ZST, assembling multiple plugins into one scheduled runtime.
    const COLLECTION: GeneratedPluginCollection<SDimpl> = build_generated_collection::<SDimpl>();
    App::new(COLLECTION).run();
}
