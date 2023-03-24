mod adb;
mod app;
mod config;
mod constants;
mod explorer;
mod inputs;
mod magic_packet;
mod monitors;
mod night_light;
mod server;
mod startup;
mod trayicon;

pub mod utils;
#[macro_use]
pub mod macros;
#[macro_use]
extern crate log;

pub use crate::app::start;
pub use crate::config::Config;
