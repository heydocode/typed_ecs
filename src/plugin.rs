#![allow(async_fn_in_trait)]

use crate::shared_data::SharedData;

pub trait Plugin<SD: SharedData> {
    // Methods are in their order of execution
    
    // APP INIT - PRE STARTUP
    fn build() -> Self;
    
    // STARTUP
    #[inline(always)]
    fn startup(&mut self, _sd: &SD) {}
    #[inline(always)]
    fn apply_startup(&mut self, _sd: &mut SD) {}
    
    #[inline(always)]
    async fn async_startup(&mut self, _sd: &SD) {}
    #[inline(always)]
    fn apply_async_startup(&mut self, _sd: &mut SD) {}
    
    // LOOP - UPDATES

    #[inline(always)]
    fn pre_update(&mut self, _sd: &SD) {}
    #[inline(always)]
    fn apply_pre_update(&mut self, _sd: &mut SD) {}
    
    #[inline(always)]
    fn update(&mut self, _sd: &SD) {}
    #[inline(always)]
    fn apply_update(&mut self, _sd: &mut SD) {}
    
    #[inline(always)]
    fn post_update(&mut self, _sd: &SD) {}
    #[inline(always)]
    fn apply_post_update(&mut self, _sd: &mut SD) {}
    
    #[inline(always)]
    async fn async_update(&mut self, _sd: &SD) {}
    #[inline(always)]
    fn apply_async_update(&mut self, _sd: &mut SD) {}
    
    #[inline(always)]
    fn exit_check(&mut self, _should_exit: &mut bool, _sd: &SD) {}
    
    // SHUTDOWN (runs once)
    
    #[inline(always)]
    fn on_exit(&mut self, _sd: &SD) {}
}
