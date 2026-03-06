//! plugin_collection.rs example
//!
//! Added imports to glue the lib with the examples
//! This should be sufficient for all examples, but if that's
//! not the case, feel free to add some, it's pretty trivial.
//! Moreover, in Godbolt you can just click on one button to add
//! a missing import
use crate::app::ShouldExit;
use crate::app::App;
use crate::plugin_collection::PluginCollection;
use crate::plugin::Plugin;
use crate::shared_data::SharedData;

// ----------------------------------------------------
// Lib code - obtainable with:
// `cargo expand --lib > lib_output.rs`
// Note that Godbolt doesn't accept `core`, so take
// care of stating `use std as core`. Moreover, you will
// have to do some other manipulations, but the Rust compiler
// in Godbolt will guide you through the process.
// ----------------------------------------------------
use std as core;
#[macro_use]
extern crate std;

pub mod app {
    use core::marker::PhantomData;
    use crate::{plugin_collection::PluginCollection, shared_data::SharedData};
    /// ECS holder field, that is distributed for all plugins
    /// that want to be able to gracefully shutdown the ECS.
    /// As soon as its value is true, all plugins have their
    /// `on_exit` method called, followed by loop breaking and
    /// then app shutdown.
    pub struct ShouldExit(bool);
    impl ShouldExit {
        /// Changes ShouldExit struct value to true,
        /// causing a graceful shutdown.
        pub fn request_exit(&mut self) {
            self.0 = true;
        }
        pub fn get_val(&self) -> bool {
            self.0
        }
    }
    pub struct App<
        SD: SharedData,
        PC: PluginCollection<SD>,
        Executor: ExecutorTrait = DefaultExecutor,
    > {
        pub executor: PhantomData<Executor>,
        pub shared_data: SD,
        pub plugin_collection: PC,
        pub should_exit: ShouldExit,
    }
    impl<SD: SharedData, PC: PluginCollection<SD>> App<SD, PC, DefaultExecutor> {
        pub fn new(plugin_collection: PC) -> App<SD, PC, DefaultExecutor> {
            App::<SD, PC, DefaultExecutor> {
                executor: PhantomData::<DefaultExecutor>,
                shared_data: SD::build(),
                plugin_collection,
                should_exit: ShouldExit(false),
            }
        }
    }
    impl<
        SD: SharedData,
        PC: PluginCollection<SD>,
        Executor: ExecutorTrait,
    > App<SD, PC, Executor> {
        pub fn new_with_executor(
            plugin_collection: PC,
            executor: PhantomData<Executor>,
        ) -> Self {
            Self {
                executor,
                shared_data: SD::build(),
                plugin_collection,
                should_exit: ShouldExit(false),
            }
        }
        pub fn run(self) {
            Executor::run(self)
        }
    }
    pub struct DefaultExecutor;
    impl ExecutorTrait for DefaultExecutor {
        fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
            app: App<SD, PC, Executor>,
        ) {
            let plugin_collection = &app.plugin_collection;
            let mut sd = app.shared_data;
            let mut should_exit = app.should_exit;
            plugin_collection.startup_ref_sd_all(&sd);
            plugin_collection.startup_mutref_sd_all(&mut sd);
            while !should_exit.get_val() {
                plugin_collection.pre_update_ref_sd_all(&sd);
                plugin_collection.pre_update_mutref_sd_all(&mut sd);
                plugin_collection.update_ref_sd_all(&sd);
                plugin_collection.update_mutref_sd_all(&mut sd);
                plugin_collection.post_update_ref_sd_all(&sd);
                plugin_collection.post_update_mutref_sd_all(&mut sd);
                plugin_collection.exit_check_all(&mut should_exit, &sd);
            }
            plugin_collection.on_exit_all(&sd);
        }
    }
    pub trait ExecutorTrait {
        fn run<SD: SharedData, PC: PluginCollection<SD>, Executor: ExecutorTrait>(
            app: App<SD, PC, Executor>,
        );
    }
}
pub mod plugin {
    use crate::{app::ShouldExit, shared_data::SharedData};
    pub trait Plugin<SD: SharedData> {
        #[inline(always)]
        fn pre_startup_ref_sd(&self, _sd: &SD) {}
        #[inline(always)]
        fn pre_startup_mutref_sd(&self, _sd: &mut SD) {}
        #[inline(always)]
        fn startup_ref_sd(&self, _sd: &SD) {}
        #[inline(always)]
        fn startup_mutref_sd(&self, _sd: &mut SD) {}
        #[inline(always)]
        fn post_startup_ref_sd(&self, _sd: &SD) {}
        #[inline(always)]
        fn post_startup_mutref_sd(&self, _sd: &mut SD) {}
        #[inline(always)]
        fn pre_update_ref_sd(&self, _sd: &SD) {}
        #[inline(always)]
        fn pre_update_mutref_sd(&self, _sd: &mut SD) {}
        #[inline(always)]
        fn update_ref_sd(&self, _sd: &SD) {}
        #[inline(always)]
        fn update_mutref_sd(&self, _sd: &mut SD) {}
        #[inline(always)]
        fn post_update_ref_sd(&self, _sd: &SD) {}
        #[inline(always)]
        fn post_update_mutref_sd(&self, _sd: &mut SD) {}
        #[inline(always)]
        fn exit_check(&self, _should_exit: &mut ShouldExit, _sd: &SD) {}
        #[inline(always)]
        fn on_exit(&self, _sd: &SD) {}
    }
}
pub mod plugin_collection {
    use crate::{app::ShouldExit, shared_data::SharedData};
    /// The generated PluginCollection implements this trait. The SharedData
    /// constraints are local to each plugin, and the SharedData of the collection
    /// itself is the specified (explicitely) SharedData instance by the user.
    ///
    /// This trait should always be implemented automatically with the dedicated
    /// macro `generate_collection!`. Implementing it manually may, and will
    /// break the program!
    pub trait PluginCollection<SD: SharedData> {
        fn pre_startup_ref_sd_all(&self, sd: &SD);
        fn pre_startup_mutref_sd_all(&self, sd: &mut SD);
        fn startup_ref_sd_all(&self, sd: &SD);
        fn startup_mutref_sd_all(&self, sd: &mut SD);
        fn post_startup_ref_sd_all(&self, sd: &SD);
        fn post_startup_mutref_sd_all(&self, sd: &mut SD);
        fn pre_update_ref_sd_all(&self, sd: &SD);
        fn pre_update_mutref_sd_all(&self, sd: &mut SD);
        fn update_ref_sd_all(&self, sd: &SD);
        fn update_mutref_sd_all(&self, sd: &mut SD);
        fn post_update_ref_sd_all(&self, sd: &SD);
        fn post_update_mutref_sd_all(&self, sd: &mut SD);
        fn exit_check_all(&self, _should_exit: &mut ShouldExit, sd: &SD);
        fn on_exit_all(&self, sd: &SD);
        /// Hook, called individually once per schedule (e.g. on the
        /// beginning of Startup, Update, ...).
        ///
        /// Used for:
        /// - profiling: `profile` feature
        ///
        /// When all of the features listed above are disabled, the
        /// function gets optimized away, as it returns a ZST.
        #[inline(always)]
        fn on_schedule_start(&self, _schedule: &'static str) -> impl Drop {
            {
                use crate::guard::NoopGuard;
                NoopGuard
            }
        }
        /// Hook, called individually once per plugin system call.
        ///
        /// Used for:
        /// - profiling: `profile` feature
        ///
        /// When all of the features listed above are disabled, the
        /// function gets optimized away, as it returns a ZST.
        #[inline(always)]
        fn on_system_start(
            &self,
            _schedule: &'static str,
            _plugin: &'static str,
            _system: &'static str,
        ) -> impl Drop {
            {
                use crate::guard::NoopGuard;
                NoopGuard
            }
        }
    }
}
pub mod shared_data {
    /// SharedData is a trait that allows for plugins to define methods
    /// they expect from the instance (via super traits on impl) and
    /// to ensure at compile-time that every loaded plugin is OK with
    /// the SharedData instance.
    ///
    /// In other words, each plugin can request the SharedData to have certain
    /// methods.
    ///
    /// Example:
    /// ```rust
    /// struct Ping {
    ///     pinged: bool
    /// }
    /// struct PingerPlugin;
    /// struct WaitingPlugin;
    /// trait SharedDataRequirements {
    ///     fn get_ping_field(&self) -> &Ping;
    ///     fn set_ping_field(&mut self, ping: &Ping);
    /// }
    ///
    /// // Here, the compiler knows that PingerPlugin requires the SharedData instance
    /// // to implement the SharedDataRequirements trait. So, if we'd attempt to add
    /// // PingerPlugin when the SharedData instance doesn't implement this trait,
    /// // we would obtain a compile-error.
    /// impl<SD: SharedData + SharedDataRequirements> Plugin<SD> for PingerPlugin {
    ///     fn build() -> Self {
    ///         Self
    ///     }
    /// }
    ///
    /// // Same thing for this plugin
    /// impl<SD: SharedData + SharedDataRequirements> Plugin<SD> for WaitingPlugin {
    ///     fn build() -> Self {
    ///         Self
    ///     }
    /// }
    ///
    /// // Note that this SharedData instance doesn't implement the
    /// // SharedDataRequirements trait!
    /// struct NonValidSharedData;
    ///
    /// impl SharedData for NonValidSharedData {
    ///     fn build() -> Self {
    ///         Self
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let app = App::new_with_SharedData::<NonValidSharedData>();
    ///     // Compile error! Plugins just can't be added into the plugin
    ///     // typed system because type requirements aren't met!
    ///     let app = app.add_plugins::<(PingerPlugin, WaitingPlugin)>();
    ///     app.build().run();
    /// }
    /// ```
    pub trait SharedData {
        fn build() -> Self;
    }
    pub struct PhantomSharedData;
    impl SharedData for PhantomSharedData {
        fn build() -> Self {
            Self
        }
    }
}
pub mod guard {
    pub struct NoopGuard;
    impl Drop for NoopGuard {
        #[inline(always)]
        fn drop(&mut self) {}
    }
}

// Note: you can safely comment this out because
// cargo expand has already expanded the macro
//pub use macros;



// -------------------- EXAMPLE CODE ----------
// cargo expand --example example > output.rs
// Please replace all expanded internal macros into
// their initial representation. Example:
// `println!(...))` -> invalid in Rustlang
// So, replace with `println!(...)` or `print!(...)`.
// Also you must make the main function public! (`pub` keyword)


// use typed_ecs::{
//     app::{App, ShouldExit},
//     macros::generate_collection, plugin::Plugin, plugin_collection::PluginCollection,
//     shared_data::SharedData,
// };
struct Plugin1;
impl<SD: SharedData> Plugin<SD> for Plugin1 {
    fn startup_ref_sd(&self, _sd: &SD) {
        {
            println!("Hello from plugin 1!\n");
        };
    }
}
struct Plugin2;
impl<SD: SharedData> Plugin<SD> for Plugin2 {}
impl Plugin2 {
    #[allow(dead_code)]
    const MARKER_STRING: &'static str = "I won't get to the final binary because of dead-code eliminations, but anyways...";
}
struct Plugin3;
impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for Plugin3 {
    fn startup_ref_sd(&self, sd: &SD) {
        {
            println!("Initial val value: {0}\n", sd.get_val());
        };
    }
    fn pre_update_mutref_sd(&self, sd: &mut SD) {
        let val = sd.get_val();
        if !(val >= u8::MAX - 1) {
            sd.set_val(val + 1);
        } else {
            sd.set_val(0);
            sd.incr_i();
        }
    }
}
struct Plugin4;
impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for Plugin4 {
    fn post_update_ref_sd(&self, sd: &SD) {
        let val = sd.get_val();
        {
            println!("Current val: {0}\n", val);
        };
    }
    fn on_exit(&self, sd: &SD) {
        let val = sd.get_val();
        let i = sd.get_i();
        {
            println!("Current val: {0}\n", val);
        };
        {
            println!("Iterations number: {0}\n", i);
        };
    }
}
struct CtrlCHandler;
impl<SD: SharedData + AdditionalRequirement> Plugin<SD> for CtrlCHandler {
    fn exit_check(&self, should_exit: &mut ShouldExit, sd: &SD) {
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

pub fn main() {
    use core::marker::PhantomData;
    struct GeneratedPluginCollection<SD> {
        ctrlchandler: CtrlCHandler,
        plugin1: Plugin1,
        plugin2: Plugin2,
        plugin3: Plugin3,
        plugin4: Plugin4,
        _marker: PhantomData<SD>,
    }
    impl<SD> PluginCollection<SD> for GeneratedPluginCollection<SD>
    where
        SD: SharedData,
        CtrlCHandler: Plugin<SD>,
        Plugin1: Plugin<SD>,
        Plugin2: Plugin<SD>,
        Plugin3: Plugin<SD>,
        Plugin4: Plugin<SD>,
    {
        #[inline(always)]
        fn pre_startup_ref_sd_all(&self, sd: &SD) {
            let _sched_guard = self.on_schedule_start("PreStartupRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreStartupRef",
                        "CtrlCHandler",
                        "pre_startup_ref_sd",
                    );
                self.ctrlchandler.pre_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreStartupRef", "Plugin1", "pre_startup_ref_sd");
                self.plugin1.pre_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreStartupRef", "Plugin2", "pre_startup_ref_sd");
                self.plugin2.pre_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreStartupRef", "Plugin3", "pre_startup_ref_sd");
                self.plugin3.pre_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreStartupRef", "Plugin4", "pre_startup_ref_sd");
                self.plugin4.pre_startup_ref_sd(sd);
            }
        }
        #[inline(always)]
        fn pre_startup_mutref_sd_all(&self, sd: &mut SD) {
            let _sched_guard = self.on_schedule_start("PreStartupMutRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreStartupMutRef",
                        "CtrlCHandler",
                        "pre_startup_mutref_sd",
                    );
                self.ctrlchandler.pre_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreStartupMutRef",
                        "Plugin1",
                        "pre_startup_mutref_sd",
                    );
                self.plugin1.pre_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreStartupMutRef",
                        "Plugin2",
                        "pre_startup_mutref_sd",
                    );
                self.plugin2.pre_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreStartupMutRef",
                        "Plugin3",
                        "pre_startup_mutref_sd",
                    );
                self.plugin3.pre_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreStartupMutRef",
                        "Plugin4",
                        "pre_startup_mutref_sd",
                    );
                self.plugin4.pre_startup_mutref_sd(sd);
            }
        }
        #[inline(always)]
        fn startup_ref_sd_all(&self, sd: &SD) {
            let _sched_guard = self.on_schedule_start("StartupRef");
            {
                let _sys_guard = self
                    .on_system_start("StartupRef", "CtrlCHandler", "startup_ref_sd");
                self.ctrlchandler.startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupRef", "Plugin1", "startup_ref_sd");
                self.plugin1.startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupRef", "Plugin2", "startup_ref_sd");
                self.plugin2.startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupRef", "Plugin3", "startup_ref_sd");
                self.plugin3.startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupRef", "Plugin4", "startup_ref_sd");
                self.plugin4.startup_ref_sd(sd);
            }
        }
        #[inline(always)]
        fn startup_mutref_sd_all(&self, sd: &mut SD) {
            let _sched_guard = self.on_schedule_start("StartupMutRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "StartupMutRef",
                        "CtrlCHandler",
                        "startup_mutref_sd",
                    );
                self.ctrlchandler.startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupMutRef", "Plugin1", "startup_mutref_sd");
                self.plugin1.startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupMutRef", "Plugin2", "startup_mutref_sd");
                self.plugin2.startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupMutRef", "Plugin3", "startup_mutref_sd");
                self.plugin3.startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("StartupMutRef", "Plugin4", "startup_mutref_sd");
                self.plugin4.startup_mutref_sd(sd);
            }
        }
        #[inline(always)]
        fn post_startup_ref_sd_all(&self, sd: &SD) {
            let _sched_guard = self.on_schedule_start("PostStartupRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostStartupRef",
                        "CtrlCHandler",
                        "post_startup_ref_sd",
                    );
                self.ctrlchandler.post_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostStartupRef", "Plugin1", "post_startup_ref_sd");
                self.plugin1.post_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostStartupRef", "Plugin2", "post_startup_ref_sd");
                self.plugin2.post_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostStartupRef", "Plugin3", "post_startup_ref_sd");
                self.plugin3.post_startup_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostStartupRef", "Plugin4", "post_startup_ref_sd");
                self.plugin4.post_startup_ref_sd(sd);
            }
        }
        #[inline(always)]
        fn post_startup_mutref_sd_all(&self, sd: &mut SD) {
            let _sched_guard = self.on_schedule_start("PostStartupMutRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostStartupMutRef",
                        "CtrlCHandler",
                        "post_startup_mutref_sd",
                    );
                self.ctrlchandler.post_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostStartupMutRef",
                        "Plugin1",
                        "post_startup_mutref_sd",
                    );
                self.plugin1.post_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostStartupMutRef",
                        "Plugin2",
                        "post_startup_mutref_sd",
                    );
                self.plugin2.post_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostStartupMutRef",
                        "Plugin3",
                        "post_startup_mutref_sd",
                    );
                self.plugin3.post_startup_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostStartupMutRef",
                        "Plugin4",
                        "post_startup_mutref_sd",
                    );
                self.plugin4.post_startup_mutref_sd(sd);
            }
        }
        #[inline(always)]
        fn pre_update_ref_sd_all(&self, sd: &SD) {
            let _sched_guard = self.on_schedule_start("PreUpdateRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreUpdateRef",
                        "CtrlCHandler",
                        "pre_update_ref_sd",
                    );
                self.ctrlchandler.pre_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreUpdateRef", "Plugin1", "pre_update_ref_sd");
                self.plugin1.pre_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreUpdateRef", "Plugin2", "pre_update_ref_sd");
                self.plugin2.pre_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreUpdateRef", "Plugin3", "pre_update_ref_sd");
                self.plugin3.pre_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PreUpdateRef", "Plugin4", "pre_update_ref_sd");
                self.plugin4.pre_update_ref_sd(sd);
            }
        }
        #[inline(always)]
        fn pre_update_mutref_sd_all(&self, sd: &mut SD) {
            let _sched_guard = self.on_schedule_start("PreUpdateMutRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreUpdateMutRef",
                        "CtrlCHandler",
                        "pre_update_mutref_sd",
                    );
                self.ctrlchandler.pre_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreUpdateMutRef",
                        "Plugin1",
                        "pre_update_mutref_sd",
                    );
                self.plugin1.pre_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreUpdateMutRef",
                        "Plugin2",
                        "pre_update_mutref_sd",
                    );
                self.plugin2.pre_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreUpdateMutRef",
                        "Plugin3",
                        "pre_update_mutref_sd",
                    );
                self.plugin3.pre_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PreUpdateMutRef",
                        "Plugin4",
                        "pre_update_mutref_sd",
                    );
                self.plugin4.pre_update_mutref_sd(sd);
            }
        }
        #[inline(always)]
        fn update_ref_sd_all(&self, sd: &SD) {
            let _sched_guard = self.on_schedule_start("UpdateRef");
            {
                let _sys_guard = self
                    .on_system_start("UpdateRef", "CtrlCHandler", "update_ref_sd");
                self.ctrlchandler.update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateRef", "Plugin1", "update_ref_sd");
                self.plugin1.update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateRef", "Plugin2", "update_ref_sd");
                self.plugin2.update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateRef", "Plugin3", "update_ref_sd");
                self.plugin3.update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateRef", "Plugin4", "update_ref_sd");
                self.plugin4.update_ref_sd(sd);
            }
        }
        #[inline(always)]
        fn update_mutref_sd_all(&self, sd: &mut SD) {
            let _sched_guard = self.on_schedule_start("UpdateMutRef");
            {
                let _sys_guard = self
                    .on_system_start("UpdateMutRef", "CtrlCHandler", "update_mutref_sd");
                self.ctrlchandler.update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateMutRef", "Plugin1", "update_mutref_sd");
                self.plugin1.update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateMutRef", "Plugin2", "update_mutref_sd");
                self.plugin2.update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateMutRef", "Plugin3", "update_mutref_sd");
                self.plugin3.update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("UpdateMutRef", "Plugin4", "update_mutref_sd");
                self.plugin4.update_mutref_sd(sd);
            }
        }
        #[inline(always)]
        fn post_update_ref_sd_all(&self, sd: &SD) {
            let _sched_guard = self.on_schedule_start("PostUpdateRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostUpdateRef",
                        "CtrlCHandler",
                        "post_update_ref_sd",
                    );
                self.ctrlchandler.post_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostUpdateRef", "Plugin1", "post_update_ref_sd");
                self.plugin1.post_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostUpdateRef", "Plugin2", "post_update_ref_sd");
                self.plugin2.post_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostUpdateRef", "Plugin3", "post_update_ref_sd");
                self.plugin3.post_update_ref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("PostUpdateRef", "Plugin4", "post_update_ref_sd");
                self.plugin4.post_update_ref_sd(sd);
            }
        }
        #[inline(always)]
        fn post_update_mutref_sd_all(&self, sd: &mut SD) {
            let _sched_guard = self.on_schedule_start("PostUpdateMutRef");
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostUpdateMutRef",
                        "CtrlCHandler",
                        "post_update_mutref_sd",
                    );
                self.ctrlchandler.post_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostUpdateMutRef",
                        "Plugin1",
                        "post_update_mutref_sd",
                    );
                self.plugin1.post_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostUpdateMutRef",
                        "Plugin2",
                        "post_update_mutref_sd",
                    );
                self.plugin2.post_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostUpdateMutRef",
                        "Plugin3",
                        "post_update_mutref_sd",
                    );
                self.plugin3.post_update_mutref_sd(sd);
            }
            {
                let _sys_guard = self
                    .on_system_start(
                        "PostUpdateMutRef",
                        "Plugin4",
                        "post_update_mutref_sd",
                    );
                self.plugin4.post_update_mutref_sd(sd);
            }
        }
        #[inline(always)]
        fn exit_check_all(&self, should_exit: &mut ShouldExit, sd: &SD) {
            let _sched_guard = self.on_schedule_start("ExitCheck");
            {
                let _sys_guard = self
                    .on_system_start("ExitCheck", "CtrlCHandler", "exit_check");
                self.ctrlchandler.exit_check(should_exit, sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("ExitCheck", "Plugin1", "exit_check");
                self.plugin1.exit_check(should_exit, sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("ExitCheck", "Plugin2", "exit_check");
                self.plugin2.exit_check(should_exit, sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("ExitCheck", "Plugin3", "exit_check");
                self.plugin3.exit_check(should_exit, sd);
            }
            {
                let _sys_guard = self
                    .on_system_start("ExitCheck", "Plugin4", "exit_check");
                self.plugin4.exit_check(should_exit, sd);
            }
        }
        #[inline(always)]
        fn on_exit_all(&self, sd: &SD) {
            let _sched_guard = self.on_schedule_start("OnExit");
            {
                let _sys_guard = self
                    .on_system_start("OnExit", "CtrlCHandler", "on_exit");
                self.ctrlchandler.on_exit(sd);
            }
            {
                let _sys_guard = self.on_system_start("OnExit", "Plugin1", "on_exit");
                self.plugin1.on_exit(sd);
            }
            {
                let _sys_guard = self.on_system_start("OnExit", "Plugin2", "on_exit");
                self.plugin2.on_exit(sd);
            }
            {
                let _sys_guard = self.on_system_start("OnExit", "Plugin3", "on_exit");
                self.plugin3.on_exit(sd);
            }
            {
                let _sys_guard = self.on_system_start("OnExit", "Plugin4", "on_exit");
                self.plugin4.on_exit(sd);
            }
        }
    }
    const fn build_generated_collection<SD>() -> GeneratedPluginCollection<SD>
    where
        SD: SharedData,
        CtrlCHandler: Plugin<SD>,
        Plugin1: Plugin<SD>,
        Plugin2: Plugin<SD>,
        Plugin3: Plugin<SD>,
        Plugin4: Plugin<SD>,
    {
        GeneratedPluginCollection::<SD> {
            ctrlchandler: CtrlCHandler,
            plugin1: Plugin1,
            plugin2: Plugin2,
            plugin3: Plugin3,
            plugin4: Plugin4,
            _marker: PhantomData,
        }
    }
    const COLLECTION: GeneratedPluginCollection<SDimpl> = build_generated_collection::<
        SDimpl,
    >();
    App::new(COLLECTION).run();
}
