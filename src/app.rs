use crate::config::Config;
use crate::explorer::kill_explorer;
use crate::startup::Startup;
use crate::trayicon::TrayIcon;
use crate::utils::{check_error, get_window_ptr, set_window_ptr, CheckError};

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use windows::core::PCWSTR;
use windows::w;
use windows::Win32::Foundation::{GetLastError, COLORREF, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
    PostQuitMessage, RegisterClassW, RegisterWindowMessageW, SetWindowLongPtrW, TranslateMessage,
    CW_USEDEFAULT, GWL_STYLE, MSG, WINDOW_STYLE, WM_COMMAND, WM_LBUTTONUP, WM_RBUTTONUP, WNDCLASSW,
    WS_CAPTION, WS_EX_TOOLWINDOW,
};

pub const NAME: PCWSTR = w!("Window Switcher");
pub const WM_USER_TRAYICON: u32 = 6000;
pub const IDM_EXIT: u32 = 1;
pub const IDM_STARTUP: u32 = 2;

const BG_COLOR: COLORREF = COLORREF(0x4c4c4c);

pub fn start(config: &Config) -> Result<()> {
    info!("start config={:?}", config);
    App::start(config)
}

/// When the taskbar is created, it registers a message with the "TaskbarCreated" string and then broadcasts this message to all top-level windows
/// When the application receives this message, it should assume that any taskbar icons it added have been removed and add them again.
static S_U_TASKBAR_RESTART: Lazy<u32> =
    Lazy::new(|| unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) });

pub struct App {
    hwnd: HWND,
    trayicon: TrayIcon,
    startup: Startup,
}

impl App {
    pub fn start(config: &Config) -> Result<()> {
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
            lpszClassName: NAME,
            hbrBackground: unsafe { CreateSolidBrush(BG_COLOR) },
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
                NAME,
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
