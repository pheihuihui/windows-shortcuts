extern crate native_windows_gui as nwg;
use std::{cell::RefCell, fs};

use constants::CONFIG_FILE;
use nwg::NativeUi;

mod main_page;
use main_page::BasicApp;

mod constants;
mod xinput_page;

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
    tray_ip: nwg::MenuItem,
    tray_cleaner: nwg::MenuItem,
    tv_ip_addr: RefCell<String>,
    tv_mac_addr: [u8; 6],
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
        nwg::modal_info_message(
            &self.window,
            "ip addr",
            self.tv_ip_addr.borrow_mut().as_ref(),
        );
    }

    fn hello2(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show(
            "Hello World",
            Some("Welcome to my application"),
            Some(flags),
            Some(&self.icon),
        );
    }

    fn get_ip_addr(self) {
        let res = fs::read_to_string(CONFIG_FILE);
        match res {
            Ok(val) => {
                // nwg::modal_info_message(&self.window, "Hello", val.as_str());
                *self.tv_ip_addr.borrow_mut() = val;
            }
            Err(_) => {
                nwg::modal_error_message(&self.window, "error", "...");
            }
        }
    }

    fn show_main_page(&self) {
        let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build Main Page");
        nwg::dispatch_thread_events();
    }

    fn clear_clipboard() {}

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
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
                .text("Main Page")
                .parent(&data.tray_menu)
                .build(&mut data.tray_main_page)?;

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
                // SystemTray::get_ip_addr(&evt_ui);
                let res = fs::read_to_string(CONFIG_FILE);
                match res {
                    Ok(val) => {
                        // nwg::modal_info_message(&self.window, "Hello", val.as_str());
                        *evt_ui.tv_ip_addr.borrow_mut() = val;
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
                            SystemTray::show_main_page(&evt_ui);
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
                                SystemTray::show_main_page(&evt_ui);
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
