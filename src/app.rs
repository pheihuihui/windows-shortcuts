use std::thread;

use crate::adb::{capture_screen_adb, connect_tv_adb};
use crate::constants::{
    APP_CONFIG, APP_NAME, IDM_CAPTURE, IDM_EXIT, IDM_MONITOR, IDM_STARTUP, IDM_TV,
    S_U_TASKBAR_RESTART, WM_USER_TRAYICON,
};
use crate::explorer::kill_explorer;
use crate::server::ShortServer;
use crate::startup::Startup;
use crate::trayicon::TrayIcon;
use crate::utils::{
    check_error, get_window_ptr, set_window_ptr, switch_to_monitor, switch_to_tv, CheckError,
};

use anyhow::{anyhow, Result};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
    PostQuitMessage, RegisterClassW, SetWindowLongPtrW, TranslateMessage, CW_USEDEFAULT, GWL_STYLE,
    MSG, WINDOW_STYLE, WM_COMMAND, WM_LBUTTONUP, WM_RBUTTONUP, WNDCLASSW, WS_CAPTION,
    WS_EX_TOOLWINDOW,
};

pub fn start_app() -> Result<()> {
    let short = ShortServer::from_config();
    thread::spawn(move || {
        short.start_server();
    });
    App::start()
}

pub struct App {
    hwnd: HWND,
    trayicon: TrayIcon,
    startup: Startup,
}

impl App {
    pub fn start() -> Result<()> {
        let hwnd = Self::create_window()?;

        let trayicon = TrayIcon::create();

        let startup = Startup::init()?;

        let mut app = App {
            hwnd,
            trayicon,
            startup,
        };

        app.set_trayicon()?;

        let app_ptr = Box::into_raw(Box::new(app)) as _;
        check_error(|| set_window_ptr(hwnd, app_ptr))
            .map_err(|err| anyhow!("Failed to set window ptr, {err}"))?;

        Self::eventloop()
    }

    fn eventloop() -> Result<()> {
        let mut message = MSG::default();
        loop {
            let ret = unsafe { GetMessageW(&mut message, HWND(0), 0, 0) };
            match ret.0 {
                -1 => {
                    unsafe { GetLastError() }.ok()?;
                }
                0 => break,
                _ => unsafe {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                },
            }
        }

        Ok(())
    }

    fn create_window() -> Result<HWND> {
        let hinstance = unsafe { GetModuleHandleW(None) }
            .map_err(|err| anyhow!("Failed to get current module handle, {err}"))?;

        let window_class = WNDCLASSW {
            hInstance: hinstance,
            lpszClassName: APP_NAME,
            hbrBackground: HBRUSH(0),
            lpfnWndProc: Some(App::window_proc),
            ..Default::default()
        };

        let atom = unsafe { RegisterClassW(&window_class) }
            .check_error()
            .map_err(|err| anyhow!("Failed to register class, {err}"))?;

        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_TOOLWINDOW,
                PCWSTR(atom as *mut u16),
                APP_NAME,
                WINDOW_STYLE(0),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                hinstance,
                None,
            )
        }
        .check_error()
        .map_err(|err| anyhow!("Failed to create windows, {err}"))?;

        // hide caption
        let mut style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) } as u32;
        style &= !WS_CAPTION.0;
        unsafe { SetWindowLongPtrW(hwnd, GWL_STYLE, style as _) };

        Ok(hwnd)
    }

    fn set_trayicon(&mut self) -> Result<()> {
        self.trayicon.register(self.hwnd)?;
        Ok(())
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match Self::handle_message(hwnd, msg, wparam, lparam) {
            Ok(ret) => ret,
            Err(err) => {
                error!("{err}");
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
        }
    }

    fn handle_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT> {
        match msg {
            WM_USER_TRAYICON => {
                let app = get_app(hwnd)?;

                let keycode = lparam.0 as u32;
                if keycode == WM_RBUTTONUP {
                    app.trayicon.show(app.startup.is_enable)?;
                }
                if keycode == WM_LBUTTONUP {
                    kill_explorer();
                }

                return Ok(LRESULT(0));
            }
            WM_COMMAND => {
                let value = wparam.0 as u32;
                let kind = ((value >> 16) & 0xffff) as u16;
                let id = value & 0xffff;
                if kind == 0 {
                    match id {
                        IDM_EXIT => {
                            if let Ok(app) = get_app(hwnd) {
                                unsafe { drop(Box::from_raw(app)) }
                            }
                            unsafe { PostQuitMessage(0) }
                        }
                        IDM_STARTUP => {
                            let app = get_app(hwnd)?;
                            app.startup.toggle()?;
                        }
                        IDM_CAPTURE => {
                            let ip = APP_CONFIG.tv_ip_addr.to_owned();
                            let dir = APP_CONFIG.screen_dir.to_owned();
                            connect_tv_adb(&ip);
                            capture_screen_adb(&dir);
                        }
                        IDM_TV => switch_to_tv(),
                        IDM_MONITOR => switch_to_monitor(),
                        _ => {}
                    }
                }
            }
            _ if msg == *S_U_TASKBAR_RESTART => {
                let app = get_app(hwnd)?;
                app.set_trayicon()?;
            }
            _ => {}
        }
        Ok(unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) })
    }
}

fn get_app(hwnd: HWND) -> Result<&'static mut App> {
    unsafe {
        let ptr = check_error(|| get_window_ptr(hwnd))
            .map_err(|err| anyhow!("Failed to get window ptr, {err}"))?;
        let tx: &mut App = &mut *(ptr as *mut _);
        Ok(tx)
    }
}
