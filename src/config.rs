use std::fs;

use crate::utils::{parse_ip_addr, parse_mac_addr};

const KEY_TV_IP: &str = "TV_IP";
const KEY_TV_MAC: &str = "TV_MAC";
const KEY_SERVER_IP: &str = "SERVER_IP";
const KEY_SERVER_PORT: &str = "PORT";
const KEY_SCREEN_DIR: &str = "SCREEN_DIR";

#[derive(Debug, Clone)]
pub struct Config {
    pub tv_ip_addr: String,
    pub tv_mac_addr: [u8; 6],
    pub server_addr: [u8; 4],
    pub server_port: String,
    pub screen_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tv_ip_addr: String::from("192.168.1.20"),
            tv_mac_addr: [1, 1, 1, 1, 1, 1],
            server_addr: [192, 168, 1, 10],
            server_port: String::from("9111"),
            screen_dir: String::from("D:\\"),
        }
    }
}

impl Config {
    pub fn load(file: &str) -> Result<Self, String> {
        let mut res = Config::default();
        let file_content = fs::read_to_string(file);
        match file_content {
            Ok(val) => {
                let lines = val.lines();
                for line in lines {
                    let arr = line.split("::").collect::<Vec<&str>>();
                    if arr.len() == 2 {
                        match arr[0] {
                            KEY_TV_IP => res.tv_ip_addr = arr[1].to_owned(),
                            KEY_TV_MAC => {
                                let mac_addr = parse_mac_addr(arr[1])?;
                                res.tv_mac_addr = mac_addr;
                            }
                            KEY_SERVER_IP => {
                                let server_ip = parse_ip_addr(arr[1])?;
                                res.server_addr = server_ip;
                            }
                            KEY_SERVER_PORT => res.server_port = arr[1].to_owned(),
                            KEY_SCREEN_DIR => res.screen_dir = arr[1].to_owned(),
                            _ => {}
                        }
                    }
                }
            }
            Err(_) => error!("failed to read config file"),
        }

        Ok(res)
    }
}
