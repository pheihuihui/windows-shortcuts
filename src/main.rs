#![cfg_attr(
    all(target_os = "windows", not(feature = "console"),),
    windows_subsystem = "windows"
)]

extern crate native_windows_gui as nwg;

use constants::CONFIG_FILE;

use monitors::set_external_display;
use nwg::NativeUi;

use server::ShortServer;

mod adb;
mod constants;
mod magic_packet;
mod monitors;
mod night_light;
mod server;

#[derive(Default)]
pub struct SystemTray {
    window: nwg::MessageWindow,
    icon: nwg::Icon,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    tray_exit: nwg::MenuItem,
    tray_switch_to_l: nwg::MenuItem,
    tray_switch_to_r: nwg::MenuItem,
    tray_hello: nwg::MenuItem,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
    use crate::monitors::set_internal_display;
    use crate::night_light::{disable_night_light, enable_night_light};

    use super::*;
    use native_windows_gui as nwg;
    use nwg::MousePressEvent;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::thread;

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
                .text("hello ")
                .parent(&data.tray_menu)
                .build(&mut data.tray_hello)?;

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

            let short_server = ShortServer::default();
            short_server.from_config_file(CONFIG_FILE);
            thread::spawn(move || {
                short_server.start_server();
            });

            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnMousePress(MousePressEvent::MousePressLeftDown) => {}
                        E::OnContextMenu => {
                            if &handle == &evt_ui.tray {
                                SystemTray::show_menu(&evt_ui);
                            }
                        }
                        E::OnMenuItemSelected => {
                            if &handle == &evt_ui.tray_exit {
                                SystemTray::exit(&evt_ui);
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
