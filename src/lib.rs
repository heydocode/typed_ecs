#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod app;
pub mod executor;
pub mod guard;
pub mod plugin;
pub mod plugin_collection;
#[cfg(feature = "profile")]
pub mod profile;
pub mod shared_data;
pub mod should_exit;

pub use futures;
#[cfg(feature = "parallel")]
pub use rayon;
pub use typed_ecs_macros as macros;
