use std::{
    io,
    process::{Command, Stdio},
};

use crate::{constants::KEYCODE_CEC_HDMI4, magic_packet::MagicPacket};

pub fn parse_mac_addr(mac: &str) -> Result<[u8; 6], &str> {
    let arr = mac.split(":").collect::<Vec<&str>>();
    let mut res: [u8; 6] = [0; 6];
    if arr.len() != 6 {
        return Err("failed 1");
    }
    for u in 0..6 {
        match u8::from_str_radix(arr[u], 16) {
            Ok(val) => {
                res[u] = val;
            }
            Err(_) => {
                return Err("failed 2");
            }
        }
    }
    Ok(res)
}

pub fn wakeup_tv_lan(mac: [u8; 6]) -> io::Result<()> {
    let mac_address = mac;
    let magic_packet = MagicPacket::new(&mac_address);
    magic_packet.send()
}

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
