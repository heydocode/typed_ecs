#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You must never enable both `std` and `no-std` at once!");

pub mod app;
pub mod executor;
pub mod guard;
pub mod plugin;
pub mod plugin_collection;
#[cfg(feature = "profile")]
pub mod profile;
pub mod shared_data;

pub use typed_ecs_macros as macros;
pub use futures;