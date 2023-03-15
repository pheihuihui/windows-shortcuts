extern crate native_windows_gui as nwg;
use std::{cell::RefCell, fs, thread, time};

use constants::CONFIG_FILE;
use magic_packet::MagicPacket;
use monitors::set_external_display;
use nwg::NativeUi;

mod xinput_page;
use adb::{connect_tv_adb, sleep_tv_adb, switch_to_port_4, wakeup_tv_adb};
use xinput_page::BasicApp;

mod adb;
mod constants;
mod magic_packet;
mod monitors;
mod night_light;

#[derive(Default)]
pub struct SystemTray {
    window: nwg::MessageWindow,
    icon: nwg::Icon,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    tray_item1: nwg::MenuItem,
    tray_item_ip: nwg::MenuItem,
    tray_exit: nwg::MenuItem,
    tray_main_page: nwg::MenuItem,
    tray_wakup_tv: nwg::MenuItem,
    tray_sleep_tv: nwg::MenuItem,
    tray_switch_to_l: nwg::MenuItem,
    tray_switch_to_r: nwg::MenuItem,
    tv_ip_addr: RefCell<String>,
    tv_mac_addr: RefCell<[u8; 6]>,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", "Hello World!");
    }

    fn say_ip_addr(&self) {
        let mac_str = format!("{:?}", self.tv_mac_addr);
        nwg::modal_info_message(&self.window, "ip addr", &mac_str);
    }

    fn wakeup_tv(&self) {
        let mac = self.tv_mac_addr.clone().into_inner();
        let ip = self.tv_ip_addr.clone().into_inner();
        thread::spawn(move || {
            let magic_p = MagicPacket::new(&mac);
            let res = magic_p.send();
            if let Ok(_) = res {
                connect_tv_adb(&ip);
                thread::sleep(time::Duration::from_millis(300));
                wakeup_tv_adb();
                thread::sleep(time::Duration::from_millis(300));
                switch_to_port_4();
            }
        });
    }

    fn sleep_tv(&self) {
        let ip = self.tv_ip_addr.clone().into_inner();
        thread::spawn(move || {
            connect_tv_adb(&ip);
            thread::sleep(time::Duration::from_millis(300));
            sleep_tv_adb();
        });
    }

    fn show_xinput_page(&self) {
        let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build Main Page");
        nwg::dispatch_thread_events();
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
    use crate::adb::parse_mac_addr;
    use crate::monitors::set_internal_display;
    use crate::night_light::{disable_night_light, enable_night_light};

    use super::*;
    use native_windows_gui as nwg;
    use nwg::MousePressEvent;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct SystemTrayUi {
        inner: Rc<SystemTray>,
        default_handler: RefCell<Vec<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<SystemTrayUi> for SystemTray {
        fn build_ui(mut data: SystemTray) -> Result<SystemTrayUi, nwg::NwgError> {
            use nwg::Event as E;

            // Resources
            nwg::Icon::builder()
                .source_file(Some("./windows.ico"))
                .build(&mut data.icon)?;

            // Controls
            nwg::MessageWindow::builder().build(&mut data.window)?;

            nwg::TrayNotification::builder()
                .parent(&data.window)
                .icon(Some(&data.icon))
                .tip(Some("Hello"))
                .build(&mut data.tray)?;

            nwg::Menu::builder()
                .popup(true)
                .parent(&data.window)
                .build(&mut data.tray_menu)?;

            nwg::MenuItem::builder()
                .text("IP...")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item_ip)?;

            nwg::MenuItem::builder()
                .text("Testing Xinput")
                .parent(&data.tray_menu)
                .build(&mut data.tray_main_page)?;

            nwg::MenuItem::builder()
                .text("Wake Up TV")
                .parent(&data.tray_menu)
                .build(&mut data.tray_wakup_tv)?;

            nwg::MenuItem::builder()
                .text("Sleep TV")
                .parent(&data.tray_menu)
                .build(&mut data.tray_sleep_tv)?;

            nwg::MenuItem::builder()
                .text("Switch to TV")
                .parent(&data.tray_menu)
                .build(&mut data.tray_switch_to_l)?;

            nwg::MenuItem::builder()
                .text("Switch to Monitor")
                .parent(&data.tray_menu)
                .build(&mut data.tray_switch_to_r)?;

            nwg::MenuItem::builder()
                .text("Exit")
                .parent(&data.tray_menu)
                .build(&mut data.tray_exit)?;

            // Wrap-up
            let ui = SystemTrayUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);

            if let Some(evt_ui) = evt_ui.upgrade() {
                // get ip addr and mac addr
                let res = fs::read_to_string(CONFIG_FILE);
                match res {
                    Ok(val) => {
                        let mut ls = val.lines();
                        let l1 = ls.next();
                        if let Some(ip_) = l1 {
                            let arr = ip_.split("::").collect::<Vec<&str>>();
                            if arr.len() == 2 {
                                *evt_ui.tv_ip_addr.borrow_mut() = arr[1].to_owned();
                            }
                        }
                        let l2 = ls.next();
                        if let Some(mac_) = l2 {
                            let arr = mac_.split("::").collect::<Vec<&str>>();
                            if arr.len() == 2 {
                                if let Ok(mac_addr) = parse_mac_addr(arr[1]) {
                                    *evt_ui.tv_mac_addr.borrow_mut() = mac_addr;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        nwg::modal_error_message(&evt_ui.window, "error", "...");
                    }
                }
            }

            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnMousePress(MousePressEvent::MousePressLeftDown) => {
                            SystemTray::show_xinput_page(&evt_ui);
                        }
                        E::OnContextMenu => {
                            if &handle == &evt_ui.tray {
                                SystemTray::show_menu(&evt_ui);
                            }
                        }
                        E::OnMenuItemSelected => {
                            if &handle == &evt_ui.tray_item1 {
                                SystemTray::say_hello(&evt_ui);
                            } else if &handle == &evt_ui.tray_item_ip {
                                SystemTray::say_ip_addr(&evt_ui);
                            } else if &handle == &evt_ui.tray_exit {
                                SystemTray::exit(&evt_ui);
                            } else if &handle == &evt_ui.tray_main_page {
                                SystemTray::show_xinput_page(&evt_ui);
                            } else if &handle == &evt_ui.tray_wakup_tv {
                                SystemTray::wakeup_tv(&evt_ui);
                            } else if &handle == &evt_ui.tray_sleep_tv {
                                SystemTray::sleep_tv(&evt_ui);
                            } else if &handle == &evt_ui.tray_switch_to_l {
                                set_external_display();
                                disable_night_light().unwrap();
                            } else if &handle == &evt_ui.tray_switch_to_r {
                                set_internal_display();
                                enable_night_light().unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            };

            ui.default_handler
                .borrow_mut()
                .push(nwg::full_bind_event_handler(
                    &ui.window.handle,
                    handle_events,
                ));

            return Ok(ui);
        }
    }

    impl Drop for SystemTrayUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for SystemTrayUi {
        type Target = SystemTray;

        fn deref(&self) -> &SystemTray {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
