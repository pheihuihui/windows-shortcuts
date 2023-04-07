#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows_shortcuts::{alert, app::start_app, utils::instance::SingleInstance};

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
