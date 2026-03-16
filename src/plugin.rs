#![allow(async_fn_in_trait)]

use crate::{app::ShouldExit, shared_data::SharedData};

pub trait Plugin<SD: SharedData> {
    #[inline(always)]
    async fn async_pre_startup_ref_sd(&self, _sd: &SD) {}
    #[inline(always)]
    async fn async_pre_startup_mutref_sd(&self, _sd: &mut SD) {}
    #[inline(always)]
    async fn async_startup_ref_sd(&self, _sd: &SD) {}
    #[inline(always)]
    async fn async_startup_mutref_sd(&self, _sd: &mut SD) {}
    #[inline(always)]
    async fn async_post_startup_ref_sd(&self, _sd: &SD) {}
    #[inline(always)]
    async fn async_post_startup_mutref_sd(&self, _sd: &mut SD) {}
    #[inline(always)]
    async fn async_pre_update_ref_sd(&self, _sd: &SD) {}
    #[inline(always)]
    async fn async_pre_update_mutref_sd(&self, _sd: &mut SD) {}
    #[inline(always)]
    async fn async_update_ref_sd(&self, _sd: &SD) {}
    #[inline(always)]
    async fn async_update_mutref_sd(&self, _sd: &mut SD) {}
    #[inline(always)]
    async fn async_post_update_ref_sd(&self, _sd: &SD) {}
    #[inline(always)]
    async fn async_post_update_mutref_sd(&self, _sd: &mut SD) {}
    #[inline(always)]
    async fn async_exit_check(&self, _should_exit: &mut ShouldExit, _sd: &SD) {}
    #[inline(always)]
    async fn async_on_exit(&self, _sd: &SD) {}

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
