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

    // ------------------------------------------------------------
    // METHODS, THE DEFAULT IMPLEMENTATION OF WHOSE SHOULD NEVER BE
    // REDEFINED BY THE USER, OR ELSE THE PROGRAM WILL NOT BEHAVE
    // AS THE LIB EXPECTS, POSSIBLY LEADING TO LOGICAL UNDEFINED
    // BEHAVIOR, THAT'S WHY THE TRAIT MUST BE IMPLEMENTED BY MACRO!
    // ------------------------------------------------------------

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
        #[cfg(feature = "profile")]
        {
            let guard = tracing::info_span!("ecs_schedule", schedule = %_schedule).entered();
            return guard;
        }
        #[cfg(not(feature = "profile"))]
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
        #[cfg(feature = "profile")]
        {
            let guard = tracing::info_span!("ecs_schedule", schedule = %_schedule, plugin = %_plugin, system = %_system).entered();
            return guard;
        }
        #[cfg(not(feature = "profile"))]
        {
            use crate::guard::NoopGuard;
            NoopGuard
        }
    }
}
