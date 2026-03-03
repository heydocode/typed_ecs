use crate::{app::ShouldExit, shared_data::SharedData};

/// The generated PluginCollection implements this trait. The SharedData
/// constraints are local to each plugin, and the SharedData of the collection
/// itself is the specified (explicitely) SharedData instance by the user.
pub trait PluginCollection<SD: SharedData> {
    fn startup_ref_sd_all(&self, sd: &SD);
    fn startup_mutref_sd_all(&self, sd: &mut SD);
    fn pre_update_ref_sd_all(&self, sd: &SD);
    fn pre_update_mutref_sd_all(&self, sd: &mut SD);
    fn update_ref_sd_all(&self, sd: &SD);
    fn update_mutref_sd_all(&self, sd: &mut SD);
    fn post_update_ref_sd_all(&self, sd: &SD);
    fn post_update_mutref_sd_all(&self, sd: &mut SD);
    fn update_exit_status_with_sd_all(&self, _should_exit: &mut ShouldExit, sd: &SD);
    fn on_exit_all(&self, sd: &SD);
}
