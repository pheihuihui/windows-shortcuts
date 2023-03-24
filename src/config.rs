use std::path::PathBuf;

use anyhow::Result;
use ini::Ini;
use log::LevelFilter;

use crate::utils::get_exe_folder;


#[derive(Debug, Clone)]
pub struct Config {
    pub trayicon: bool,
    pub log_level: LevelFilter,
    pub log_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            trayicon: true,
            log_level: LevelFilter::Info,
            log_file: None,
        }
    }
}

impl Config {
    pub fn load(ini_conf: &Ini) -> Result<Self> {
        let mut conf = Config::default();
        if let Some(section) = ini_conf.section(None::<String>) {
            if let Some(v) = section.get("trayicon").and_then(Config::to_bool) {
                conf.trayicon = v;
            }
        }

        if let Some(section) = ini_conf.section(Some("log")) {
            if let Some(level) = section.get("level").and_then(|v| v.parse().ok()) {
                conf.log_level = level;
            }
            if let Some(path) = section.get("path") {
                if !path.trim().is_empty() {
                    let mut path = PathBuf::from(path);
                    if !path.is_absolute() {
                        let parent = get_exe_folder()?;
                        path = parent.join(path);
                    }
                    conf.log_file = Some(path);
                }
            }
        }

        Ok(conf)
    }

    pub fn to_bool(v: &str) -> Option<bool> {
        match v {
            "yes" | "true" | "on" | "1" => Some(true),
            "no" | "false" | "off" | "0" => Some(false),
            _ => None,
        }
    }
}
