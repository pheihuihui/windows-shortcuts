#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{bail, Result};

use windows_shortcuts::constants::CONFIG_FILE;
use windows_shortcuts::{alert, start, utils::SingleInstance, Config};

fn main() {
    if let Err(err) = run() {
        alert!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = Config::load(CONFIG_FILE);
    let instance = SingleInstance::create("WindowSwitcherMutex")?;
    if !instance.is_single() {
        bail!("Another instance is running. This instance will abort.")
    }
    start(&config.unwrap())
}
