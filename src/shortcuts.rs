use std::{sync::OnceLock, thread, time};

use crate::{
    alert,
    constants::APP_CONFIG,
    screen::{modes::CaptureMode, take_screenshot_for_windows},
    utils::{
        adb::{
            capture_screen_adb, connect_tv_adb, sleep_tv_adb, switch_to_home, switch_to_port_4,
            wakeup_tv_adb,
        },
        explorer::kill_explorer,
        inputs::close_top_window,
        magic_packet::MagicPacket,
        monitors::{set_external_display, set_internal_display},
        night_light::disable_night_light,
    },
};

#[derive(Clone)]
pub struct Shortcut {
    pub id: Option<usize>,
    pub func: fn() -> (),
    pub is_left_click: bool,
    pub menu_name: Option<String>,
    pub web_req_url: Option<String>,
}

pub static SHORTCUTS: OnceLock<Vec<Shortcut>> = OnceLock::new();

pub fn build_shortcuts() {
    thread::spawn(|| {
        let _ = SHORTCUTS.get_or_init(|| {
            vec![
                Shortcut {
                    id: Some(8),
                    func: || {
                        let txt = APP_CONFIG.get().unwrap().screen_dir.to_owned();
                        alert!("{}", txt);
                    },
                    is_left_click: false,
                    menu_name: Some("Test".to_string()),
                    web_req_url: Some("/test_connection".to_string()),
                },
                Shortcut {
                    id: Some(9),
                    func: || {
                        let ip = APP_CONFIG.get().unwrap().tv_ip_addr.to_owned();
                        let dir = APP_CONFIG.get().unwrap().screen_dir.to_owned();
                        connect_tv_adb(&ip);
                        capture_screen_adb(&dir);
                    },
                    is_left_click: false,
                    menu_name: Some("Capture Screen".to_string()),
                    web_req_url: Some("/capture_screen".to_string()),
                },
                Shortcut {
                    id: Some(19),
                    func: || {
                        let dir = APP_CONFIG.get().unwrap().screen_dir.to_owned();
                        let _ = take_screenshot_for_windows(&dir, CaptureMode::Primary);
                    },
                    is_left_click: false,
                    menu_name: Some("Capture Windows Screen".to_string()),
                    web_req_url: Some("/capture_windows_screen".to_string()),
                },
                Shortcut {
                    id: Some(10),
                    func: || {
                        let mac = APP_CONFIG.get().unwrap().tv_mac_addr;
                        let ip = &APP_CONFIG.get().unwrap().tv_ip_addr;
                        thread::spawn(move || {
                            let magic_p = MagicPacket::new(&mac);
                            let res = magic_p.send();
                            if let Ok(_) = res {
                                thread::sleep(time::Duration::from_millis(1000));
                                connect_tv_adb(ip);
                                thread::sleep(time::Duration::from_millis(200));
                                wakeup_tv_adb();
                                thread::sleep(time::Duration::from_millis(200));
                                switch_to_port_4();
                                thread::sleep(time::Duration::from_millis(200));
                                set_external_display();
                                disable_night_light().unwrap();
                            }
                        });
                    },
                    is_left_click: false,
                    menu_name: Some("Switch to TV".to_string()),
                    web_req_url: Some("/switch_to_tv".to_string()),
                },
                Shortcut {
                    id: Some(11),
                    func: || {
                        let ip = &APP_CONFIG.get().unwrap().tv_ip_addr;
                        thread::spawn(move || {
                            connect_tv_adb(&ip);
                            thread::sleep(time::Duration::from_millis(200));
                            switch_to_home();
                            thread::sleep(time::Duration::from_millis(200));
                            // enable_night_light().unwrap();
                            set_internal_display();
                            sleep_tv_adb();
                        });
                    },
                    is_left_click: false,
                    menu_name: Some("Switch to Monitor".to_string()),
                    web_req_url: Some("/switch_to_monitor".to_string()),
                },
                Shortcut {
                    id: None,
                    func: || kill_explorer(),
                    is_left_click: true,
                    menu_name: None,
                    web_req_url: None,
                },
                Shortcut {
                    id: None,
                    func: || close_top_window(),
                    is_left_click: false,
                    menu_name: None,
                    web_req_url: Some("/close_top_window".to_string()),
                },
            ]
        });
    })
    .join()
    .unwrap();
}
