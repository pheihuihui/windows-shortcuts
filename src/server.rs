use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread, time,
};

use crate::{
    adb::{connect_tv_adb, sleep_tv_adb, switch_to_port_4, wakeup_tv_adb},
    inputs::switch_windows,
    magic_packet::MagicPacket,
    monitors::{set_external_display, set_internal_display},
    night_light::{disable_night_light, enable_night_light},
    utils::parse_mac_addr,
};

pub struct ShortServer {
    listener: Arc<TcpListener>,
    tv_ip_addr: Arc<Mutex<String>>,
    tv_mac_addr: Arc<Mutex<[u8; 6]>>,
}

impl ShortServer {
    pub fn from_config_file(&self, file: &str) {
        let res = fs::read_to_string(file);
        match res {
            Ok(val) => {
                let mut ls = val.lines();
                let l1 = ls.next();
                if let Some(ip_) = l1 {
                    let arr = ip_.split("::").collect::<Vec<&str>>();
                    if arr.len() == 2 {
                        let mut ip_addr = self.tv_ip_addr.lock().unwrap();
                        *ip_addr = arr[1].to_owned();
                    }
                }
                let l2 = ls.next();
                if let Some(mac_) = l2 {
                    let arr = mac_.split("::").collect::<Vec<&str>>();
                    if arr.len() == 2 {
                        if let Ok(mac_addr) = parse_mac_addr(arr[1]) {
                            let mut mac_addr_ = self.tv_mac_addr.lock().unwrap();
                            *mac_addr_ = mac_addr;
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    pub fn start_server(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(val) => self.handle_connection(val),
                Err(_) => {}
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        let urls: Vec<_> = http_request
            .clone()
            .into_iter()
            .filter(|x| x.starts_with("HEAD") || x.starts_with("GET"))
            .collect();
        if urls.len() == 1 {
            let url = &urls[0];
            let url: Vec<_> = url.split(" ").collect();
            if url.len() >= 2 {
                let url = url[1];
                match url {
                    "/switch_to_tv" => {
                        self.switch_to_tv();
                        disable_night_light().unwrap();
                    }
                    "/switch_to_monitor" => {
                        enable_night_light().unwrap();
                        set_internal_display();
                        self.sleep_tv();
                    }
                    "/switch_windows" => {
                        switch_windows();
                    }
                    "/hello" => {
                        println!("hello world");
                    }
                    _ => {}
                }
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }

    fn switch_to_tv(&self) {
        let mac = self.tv_mac_addr.lock().unwrap().clone();
        let ip = self.tv_ip_addr.lock().unwrap().clone();
        thread::spawn(move || {
            let magic_p = MagicPacket::new(&mac);
            let res = magic_p.send();
            if let Ok(_) = res {
                connect_tv_adb(&ip);
                thread::sleep(time::Duration::from_millis(200));
                wakeup_tv_adb();
                thread::sleep(time::Duration::from_millis(200));
                switch_to_port_4();
                thread::sleep(time::Duration::from_millis(200));
                set_external_display();
            }
        });
    }

    fn sleep_tv(&self) {
        let ip = self.tv_ip_addr.lock().unwrap().clone();
        thread::spawn(move || {
            connect_tv_adb(&ip);
            thread::sleep(time::Duration::from_millis(300));
            sleep_tv_adb();
        });
    }
}

impl Default for ShortServer {
    fn default() -> ShortServer {
        let listener = TcpListener::bind("0.0.0.0:9111").unwrap();
        let val: ShortServer = ShortServer {
            listener: Arc::new(listener),
            tv_ip_addr: Arc::new(Mutex::new("2.2.2.2".to_owned())),
            tv_mac_addr: Arc::new(Mutex::new([0, 0, 0, 0, 0, 0])),
        };
        val
    }
}
