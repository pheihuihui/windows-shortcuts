use std::{cell::RefCell, fs};

use crate::utils::parse_mac_addr;

#[derive(Debug, Clone)]
pub struct Config {
    pub tv_ip_addr: RefCell<String>,
    pub tv_mac_addr: RefCell<[u8; 6]>,
    pub server_port: RefCell<String>,
    pub screen_dir: RefCell<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tv_ip_addr: RefCell::new("1.1.1.1".to_string()),
            tv_mac_addr: RefCell::new([1, 1, 1, 1, 1, 1]),
            server_port: RefCell::new("9111".to_string()),
            screen_dir: RefCell::new("D:\\".to_string()),
        }
    }
}

impl Config {
    pub fn load(file: &str) -> Result<Self, String> {
        let res = Config::default();
        let file_content = fs::read_to_string(file);
        match file_content {
            Ok(val) => {
                let mut lines = val.lines();
                let l1 = lines.next();
                if let Some(ip_) = l1 {
                    let arr = ip_.split("::").collect::<Vec<&str>>();
                    if arr.len() == 2 {
                        *res.tv_ip_addr.borrow_mut() = arr[1].to_owned();
                    }
                }
                let l2 = lines.next();
                if let Some(mac_) = l2 {
                    let arr = mac_.split("::").collect::<Vec<&str>>();
                    if arr.len() == 2 {
                        if let Ok(mac_addr) = parse_mac_addr(arr[1]) {
                            *res.tv_mac_addr.borrow_mut() = mac_addr;
                        }
                    }
                }
                let l3 = lines.next();
                if let Some(port_) = l3 {
                    let arr = port_.split("::").collect::<Vec<&str>>();
                    if arr.len() == 2 {
                        *res.server_port.borrow_mut() = arr[1].to_owned();
                    }
                }
                let l4 = lines.next();
                if let Some(dir_) = l4 {
                    let arr = dir_.split("::").collect::<Vec<&str>>();
                    if arr.len() == 2 {
                        *res.screen_dir.borrow_mut() = arr[1].to_owned();
                    }
                }
            }
            Err(_) => error!("failed to read config file"),
        }

        Ok(res)
    }
}
