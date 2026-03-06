use crate::{app::ShouldExit, shared_data::SharedData};

pub trait Plugin<SD: SharedData> {
    #[inline(always)]
    fn startup_ref_sd(&self, _sd: &SD) {}
    #[inline(always)]
    fn startup_mutref_sd(&self, _sd: &mut SD) {}
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