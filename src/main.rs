// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::start_app;
use utils::instance::SingleInstance;

pub mod app;
pub mod config;
pub mod constants;
pub mod screen;
pub mod server;
pub mod shortcuts;
pub mod startup;
pub mod trayicon;
pub mod utils;

fn main() {
    if let Err(err) = run() {
        alert!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let instance = SingleInstance::create("WindowSwitcherMutex")?;
    if !instance.is_single() {
        return Err("Another instance is running. This instance will abort.".to_string());
    }
    start_app()
}
