#![no_std]

pub mod app;
pub mod executor;
pub mod guard;
pub mod plugin;
pub mod plugin_collection;
#[cfg(feature = "profile")]
pub mod profile;
pub mod shared_data;

pub use futures;
pub use typed_ecs_macros as macros;
#[cfg(feature = "parallel")]
pub use rayon;