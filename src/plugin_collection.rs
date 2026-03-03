use crate::{app::ShouldExit, shared_data::SharedData};

/// The generated PluginCollection implements this trait. The SharedData
/// constraints are local to each plugin.
/// 
/// TODO! post_ref_sd_update_all -> post_update_ref_sd_all
/// (And do this for all other namings -> more consistent)
/// 
/// TODO! Make a global SD, constrained by each plugin
///     -> Use previous plugingroups.
pub trait PluginCollection<SD: SharedData> {
    fn startup_ref_sd_all(&self, sd: &SD);
    fn startup_mutref_sd_all(&self, sd: &mut SD);
    fn pre_update_ref_sd_all(&self, sd: &SD);
    fn pre_update_mutref_sd_all(&self, sd: &mut SD);
    fn update_ref_sd_all(&self, sd: &SD);
    fn update_mutref_sd_all(&self, sd: &mut SD);
    fn post_ref_sd_update_all(&self, sd: &SD);
    fn post_mutref_sd_update_all(&self, sd: &mut SD);
    fn update_exit_status_with_sd_all(&self, _should_exit: &mut ShouldExit, sd: &SD);
    fn on_exit_all(&self, sd: &SD);
}
