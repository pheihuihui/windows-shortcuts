#![allow(unused)]

use std::sync::OnceLock;

use windows::core::{PCWSTR, w};

use crate::config::Config;

pub const APP_NAME: PCWSTR = w!("Windows Shortcuts");
pub const WM_USER_TRAYICON: u32 = 6000;
pub const IDM_EXIT: u32 = 1;
pub const IDM_CAPTURE: u32 = 2;
pub const IDM_STARTUP: u32 = 3;
pub const IDM_TV: u32 = 4;
pub const IDM_MONITOR: u32 = 5;

pub const KEYCODE_WAKEUP: &str = "KEYCODE_WAKEUP";
pub const KEYCODE_SLEEP: &str = "KEYCODE_SLEEP";
pub const KEYCODE_HOME: &str = "KEYCODE_HOME";
pub const KEYCODE_CEC_HDMI1: i16 = 243;
pub const KEYCODE_CEC_HDMI2: i16 = 244;
pub const KEYCODE_CEC_HDMI3: i16 = 245;
pub const KEYCODE_CEC_HDMI4: i16 = 246;

pub static APP_CONFIG: OnceLock<Config> = OnceLock::new();

/// When the taskbar is created, it registers a message with the "TaskbarCreated" string and then broadcasts this message to all top-level windows
/// When the application receives this message, it should assume that any taskbar icons it added have been removed and add them again.
pub static S_U_TASKBAR_RESTART: OnceLock<u32> = OnceLock::new();
