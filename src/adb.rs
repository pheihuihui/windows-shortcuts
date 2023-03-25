use std::process::{Command, Stdio};

use crate::constants::KEYCODE_CEC_HDMI4;

pub fn wakeup_tv_adb() {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("keyevent")
        .arg("KEYCODE_WAKEUP")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to wake up tv");
}

pub fn sleep_tv_adb() {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("keyevent")
        .arg("KEYCODE_SLEEP")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to sleep tv");
}

pub fn connect_tv_adb(ip: &str) {
    Command::new("adb")
        .arg("connect")
        .arg(ip)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to connect tv");
}

pub fn switch_to_port_4() {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("keyevent")
        .arg(format!("{:?}", KEYCODE_CEC_HDMI4))
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to sleep tv");
}
