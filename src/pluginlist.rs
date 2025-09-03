use std::marker::PhantomData;

use crate::{app::ShouldExit, plugin::Plugin, shared_data::SharedData};

pub trait PluginList<SD: SharedData> {
    fn build_all() -> Self;
}

pub trait UnallocatedPluginList<SD: SharedData>: PluginList<SD> {
    type Allocated: AllocatedPluginList<SD>;

    /// Appends a plugin in LIFO order to the current PluginList
    fn add_plugin<P: Plugin<SD>>(&self, _plugin: P) -> PhantomData<(P, Self)>
    where
        Self: UnallocatedPluginList<SD>;
    /// Allocates the PluginList: instead of allocating on each plugin add,
    /// allocate only when you've added all of your plugins.
    fn build() -> Self::Allocated;
}

pub trait AllocatedPluginList<SD: SharedData>: PluginList<SD> {
    /// Recursively calls `Plugin::build` for each Plugin of the PluginList.
    /// In release mode, should be as performant as manual per-plugin method invocation.
    fn instantiate_all<P: Plugin<SD>, PL: PluginList<SD> + AllocatedPluginList<SD>>() -> Self;
    /// Recursively calls `startup` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn startup_all(&mut self);
    /// Recursively calls `pre_update` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn pre_update_all(&mut self);
    /// Recursively calls `update_all` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn update_all(&mut self);
    /// Recursively calls `post_update` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn post_update_all(&mut self);
    /// Recursively calls `access_ref_sd` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn access_ref_sd_all(&mut self, _sd: &SD);
    /// Recursively calls `access_mutref_sd` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn access_mutref_sd_all(&mut self, _sd: &mut SD);
    /// Recursively calls `update_exit_status` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn update_exit_status_all(&self, _should_exit: &mut ShouldExit);
    /// Recursively calls `update_exit_status_with_sd` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn update_exit_status_with_sd_all(&self, _should_exit: &mut ShouldExit, _sd: &SD);
    /// Recursively calls `on_exit` method for each Plugin of the PluginList.
    /// This is using Head/Tail recursion, which is optimized away by the compiler.
    /// So in release mode, this must be as performant as manual per-plugin method invocation.
    fn on_exit_all(&mut self);
}

impl<SD: SharedData> PluginList<SD> for PhantomData<()> {
    fn build_all() -> Self {
        PhantomData
    }
}

impl<SD: SharedData> PluginList<SD> for () {
    fn build_all() -> Self {
        ()
    }
}

impl<SD: SharedData> UnallocatedPluginList<SD> for PhantomData<()> {
    type Allocated = ();

    fn add_plugin<P: Plugin<SD>>(&self, _plugin: P) -> PhantomData<(P, Self)>
    where
        Self: UnallocatedPluginList<SD>,
    {
        PhantomData
    }

    fn build() -> Self::Allocated {
        ()
    }
}

impl<SD: SharedData> AllocatedPluginList<SD> for () {
    fn instantiate_all<P: Plugin<SD>, PL: PluginList<SD> + AllocatedPluginList<SD>>() -> Self {
        ()
    }

    fn startup_all(&mut self) {}

    fn pre_update_all(&mut self) {}

    fn update_all(&mut self) {}

    fn post_update_all(&mut self) {}

    fn access_ref_sd_all(&mut self, _sd: &SD) {}

    fn access_mutref_sd_all(&mut self, _sd: &mut SD) {}

    fn update_exit_status_all(&self, _should_exit: &mut ShouldExit) {}

    fn update_exit_status_with_sd_all(&self, _should_exit: &mut ShouldExit, _sd: &SD) {}

    fn on_exit_all(&mut self) {}
}

impl<SD: SharedData, Head: Plugin<SD>, Tail: PluginList<SD>> PluginList<SD>
    for PhantomData<(Head, Tail)>
{
    fn build_all() -> Self {
        PhantomData
    }
}

impl<SD: SharedData, Head: Plugin<SD>, Tail: UnallocatedPluginList<SD>> UnallocatedPluginList<SD>
    for PhantomData<(Head, Tail)>
{
    type Allocated = (Head, Tail::Allocated);

    fn add_plugin<P: Plugin<SD>>(&self, _plugin: P) -> PhantomData<(P, Self)>
    where
        Self: UnallocatedPluginList<SD>,
    {
        PhantomData
    }

    fn build() -> Self::Allocated {
        (Head::build(), Tail::build())
    }
}

impl<SD: SharedData, Head: Plugin<SD>, Tail: PluginList<SD>> PluginList<SD> for (Head, Tail) {
    fn build_all() -> Self {
        (Head::build(), Tail::build_all())
    }
}

impl<SD: SharedData, Head: Plugin<SD>, Tail: PluginList<SD> + AllocatedPluginList<SD>>
    AllocatedPluginList<SD> for (Head, Tail)
{
    /// Recursively calls `Plugin::build` for each Plugin of the PluginList.
    /// In release mode, should be as performant as manual per-plugin method invocation.
    fn instantiate_all<P: Plugin<SD>, PL: PluginList<SD> + AllocatedPluginList<SD>>() -> Self {
        (Head::build(), Tail::build_all())
    }

    fn startup_all(&mut self) {
        self.0.startup();
        self.1.startup_all();
    }

    fn pre_update_all(&mut self) {
        self.0.pre_update();
        self.1.pre_update_all();
    }

    fn update_all(&mut self) {
        self.0.update();
        self.1.update_all();
    }

    fn post_update_all(&mut self) {
        self.0.post_update();
        self.1.post_update_all();
    }

    fn access_ref_sd_all(&mut self, sd: &SD) {
        self.0.access_ref_sd(sd);
        self.1.access_ref_sd_all(sd);
    }

    fn access_mutref_sd_all(&mut self, sd: &mut SD) {
        self.0.access_mutref_sd(sd);
        self.1.access_mutref_sd_all(sd);
    }

    fn update_exit_status_all(&self, should_exit: &mut ShouldExit) {
        self.0.update_exit_status(should_exit);
        self.1.update_exit_status_all(should_exit);
    }

    fn update_exit_status_with_sd_all(&self, should_exit: &mut ShouldExit, sd: &SD) {
        self.0.update_exit_status_with_sd(should_exit, sd);
        self.1.update_exit_status_with_sd_all(should_exit, sd);
    }

    fn on_exit_all(&mut self) {
        self.0.on_exit();
        self.1.on_exit_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::marker::PhantomData;

    // Minimal SharedData and Plugin for testing
    struct DummySharedData;
    impl SharedData for DummySharedData {
        fn build() -> Self {
            Self
        }
    }

    #[derive(Debug, PartialEq)]
    struct ExamplePlugin;
    impl Plugin<DummySharedData> for ExamplePlugin {
        fn build() -> Self {
            Self
        }
    }

    #[derive(Debug, PartialEq)]
    struct ExamplePlugin2;
    impl Plugin<DummySharedData> for ExamplePlugin2 {
        fn build() -> Self {
            Self
        }
    }

    #[test]
    fn pluginlist_instantiation() {
        // The tail must itself be an UnallocatedPluginList, so end it with PhantomData<()>.
        type Unallocated = PhantomData<(
            ExamplePlugin,
            PhantomData<(ExamplePlugin2, PhantomData<()>)>,
        )>;
        type ExpectedAllocated = (ExamplePlugin, (ExamplePlugin2, ()));

        let allocated: <Unallocated as UnallocatedPluginList<DummySharedData>>::Allocated =
            <Unallocated as UnallocatedPluginList<DummySharedData>>::build();

        let _: ExpectedAllocated = allocated;
        assert_eq!(
            allocated,
            (ExamplePlugin::build(), (ExamplePlugin2::build(), ())),
            "error!"
        )
    }

    #[test]
    fn plugins_orderings() {
        let plugins: PhantomData<(ExamplePlugin, PhantomData<()>)> =
            PhantomData::<()>.add_plugin(ExamplePlugin);
        let second_plugins: PhantomData<(
            ExamplePlugin2,
            PhantomData<(ExamplePlugin, PhantomData<()>)>,
        )> = plugins.add_plugin(ExamplePlugin2);

        assert_eq!(plugins, PhantomData::<(ExamplePlugin, PhantomData<()>)>);
        assert_eq!(
            second_plugins,
            PhantomData::<(
                ExamplePlugin2,
                PhantomData<(ExamplePlugin, PhantomData<()>)>
            )>
        );
    }

    /// This test ensures the compiler accepts the void type as a plugin.
    #[test]
    fn void_plugins() {
        let base_plugins = PhantomData::<()>.add_plugin(ExamplePlugin);
        let new_plugins = base_plugins.add_plugin(()).add_plugin(());
        let _ = new_plugins.add_plugin(());
    }
}
