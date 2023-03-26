#![allow(dead_code)]

use windows::{core::PCWSTR, w};

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

pub const CONFIG_FILE: &str = "config.txt";

// adb shell input keyevent 82
