/// SharedData is a trait that allows for plugins to define methods
/// they expect from the instance (via super traits on impl) and
/// to ensure at compile-time that every loaded plugin is OK with
/// the SharedData instance.
/// 
/// In other words, each plugin can request the SharedData to have certain
/// methods, if it needs to talk with another plugin via the SharedData instance.
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