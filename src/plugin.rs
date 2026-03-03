use core::marker::PhantomData;
use macros::assemble_collection;

use crate::{app::ShouldExit, plugin_collection::PluginCollection, shared_data::SharedData};

pub trait Plugin<SD: SharedData> {
    #[inline(always)]
    fn startup_ref_sd(&self, sd: &SD) {}
    #[inline(always)]
    fn startup_mutref_sd(&self, sd: &mut SD) {}
    #[inline(always)]
    fn pre_update_ref_sd(&self, sd: &SD) {}
    #[inline(always)]
    fn pre_update_mutref_sd(&self, sd: &mut SD) {}
    #[inline(always)]
    fn update_ref_sd(&self, sd: &SD) {}
    #[inline(always)]
    fn update_mutref_sd(&self, sd: &mut SD) {}
    #[inline(always)]
    fn post_update_ref_sd(&self, sd: &SD) {}
    #[inline(always)]
    fn post_update_mutref_sd(&self, sd: &mut SD) {}
    #[inline(always)]
    fn update_exit_status_with_sd(&self, _should_exit: &mut ShouldExit, sd: &SD) {}
    #[inline(always)]
    fn on_exit(&self, sd: &SD) {}
}

struct Plugin1;
impl<SD: SharedData> Plugin<SD> for Plugin1 {}
struct Plugin2;
impl<SD: SharedData> Plugin<SD> for Plugin2 {}
struct Plugin3;
impl<SD: SharedData> Plugin<SD> for Plugin3 {}

fn example() {
    let plugin_group = assemble_collection!(Plugin1, Plugin2, Plugin3);
}
