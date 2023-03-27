#![allow(unused)]

use std::{
    fs,
    process::{Command, Stdio},
    time::SystemTime,
};

use crate::constants::{KEYCODE_CEC_HDMI4, KEYCODE_HOME, KEYCODE_SLEEP, KEYCODE_WAKEUP};

pub fn wakeup_tv_adb() {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("keyevent")
        .arg(KEYCODE_WAKEUP)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to wake up tv");
}

pub fn sleep_tv_adb() {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("keyevent")
        .arg(KEYCODE_SLEEP)
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
        .arg(KEYCODE_CEC_HDMI4.to_string())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to sleep tv");
}

pub fn switch_to_home() {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("keyevent")
        .arg(KEYCODE_HOME)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to switch to home");
}

pub fn capture_screen(dir: &str) {
    let time = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut file_name = dir.to_string();
    file_name.push_str(r"\");
    file_name.push_str(&time.to_string());
    file_name.push_str(".png");
    let output = Command::new("adb")
        .arg("exec-out")
        .arg("screencap")
        .arg("-p")
        .output()
        .expect("Failed to execute command");

    fs::write(file_name, output.stdout).expect("Unable to write file");
}

pub fn reconnect_offline() {
    Command::new("adb")
        .arg("reconnect")
        .arg("offline")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to reconnect offline");
}
