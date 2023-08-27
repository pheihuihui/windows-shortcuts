use crate::config::Config;
use crate::constants::{
    APP_CONFIG, APP_NAME, IDM_EXIT, IDM_STARTUP, S_U_TASKBAR_RESTART, WM_USER_TRAYICON,
};
use crate::server::ShortServer;
use crate::shortcuts::{build_shortcuts, Shortcut, SHORTCUTS};
use crate::startup::Startup;
use crate::trayicon::TrayIcon;

use crate::utils::errors::{check_error, CheckError};
use crate::utils::others::{get_exe_folder, get_window_ptr, set_window_ptr};
use std::collections::HashMap;
use std::thread;

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
    PostQuitMessage, RegisterClassW, RegisterWindowMessageW, SetWindowLongPtrW, TranslateMessage,
    CW_USEDEFAULT, GWL_STYLE, MSG, WINDOW_STYLE, WM_COMMAND, WM_LBUTTONUP, WM_RBUTTONUP, WNDCLASSW,
    WS_CAPTION, WS_EX_TOOLWINDOW,
};

pub fn start_app() -> Result<(), String> {
    let _ =
        S_U_TASKBAR_RESTART.get_or_init(|| unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) });
    let _ = APP_CONFIG.get_or_init(|| {
        let mut path = get_exe_folder().unwrap();
        path.push("config");
        path.set_extension("txt");
        let dir = path.to_str().unwrap();
        Config::load(dir).unwrap()
    });
    build_shortcuts();
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
    menu_shortcuts: HashMap<usize, fn() -> ()>,
}

impl App {
    pub fn start() -> Result<(), String> {
        let hwnd = Self::create_window()?;

        let trayicon = TrayIcon::create();

        let startup = Startup::init()?;

        let mut menu_shortcuts = HashMap::new();
        let scs = SHORTCUTS
            .get()
            .unwrap()
            .to_vec()
            .into_iter()
            .filter(|x| x.id.is_some() && x.menu_name.is_some())
            .collect::<Vec<Shortcut>>();
        for ele in scs {
            menu_shortcuts.insert(ele.id.unwrap(), ele.func);
        }

        let mut app = App {
            hwnd,
            trayicon,
            startup,
            menu_shortcuts,
        };

        app.set_trayicon()?;

        let app_ptr = Box::into_raw(Box::new(app)) as _;
        check_error(|| set_window_ptr(hwnd, app_ptr))
            .map_err(|err| format!("Failed to set window ptr, {err}"))?;

        Self::eventloop()
    }

    fn eventloop() -> Result<(), String> {
        let mut message = MSG::default();
        loop {
            let ret = unsafe { GetMessageW(&mut message, HWND(0), 0, 0) };
            match ret.0 {
                -1 => {
                    let _ = unsafe { GetLastError() };
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

    fn create_window() -> Result<HWND, String> {
        let hinstance = unsafe { GetModuleHandleW(None) }
            .map_err(|err| format!("Failed to get current module handle, {err}"))?;

        let window_class = WNDCLASSW {
            hInstance: hinstance.into(),
            lpszClassName: APP_NAME,
            hbrBackground: HBRUSH(0),
            lpfnWndProc: Some(App::window_proc),
            ..Default::default()
        };

        let atom = unsafe { RegisterClassW(&window_class) }
            .check_error()
            .map_err(|err| format!("Failed to register class, {err}"))?;

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
        .map_err(|err| format!("Failed to create windows, {err}"))?;

        // hide caption
        let mut style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) } as u32;
        style &= !WS_CAPTION.0;
        unsafe { SetWindowLongPtrW(hwnd, GWL_STYLE, style as _) };

        Ok(hwnd)
    }

    fn set_trayicon(&mut self) -> Result<(), String> {
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
                println!("{:?}", err);
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
        }
    }

    fn handle_message(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<LRESULT, String> {
        match msg {
            WM_USER_TRAYICON => {
                let app = get_app(hwnd)?;

                let keycode = lparam.0 as u32;
                if keycode == WM_RBUTTONUP {
                    app.trayicon.show(app.startup.is_enable)?;
                }
                if keycode == WM_LBUTTONUP {
                    let scs = SHORTCUTS
                        .get()
                        .unwrap()
                        .to_vec()
                        .into_iter()
                        .filter(|x| x.is_left_click)
                        .collect::<Vec<Shortcut>>();
                    for ele in scs {
                        let func = ele.func;
                        func();
                    }
                }

                return Ok(LRESULT(0));
            }
            WM_COMMAND => {
                let value = wparam.0 as u32;
                let kind = ((value >> 16) & 0xffff) as u16;
                let id = value & 0xffff;
                if kind == 0 {
                    let app = get_app(hwnd)?;
                    let id_usize = usize::try_from(id).unwrap();
                    let func = app.menu_shortcuts[&id_usize];
                    func();
                    match id {
                        IDM_EXIT => {
                            if let Ok(app) = get_app(hwnd) {
                                unsafe { drop(Box::from_raw(app)) }
                            }
                            unsafe { PostQuitMessage(0) }
                        }
                        IDM_STARTUP => {
                            app.startup.toggle()?;
                        }
                        _ => {}
                    }
                }
            }
            _ if msg == *S_U_TASKBAR_RESTART.get().unwrap() => {
                let app = get_app(hwnd)?;
                app.set_trayicon()?;
            }
            _ => {}
        }
        Ok(unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) })
    }
}

fn get_app(hwnd: HWND) -> Result<&'static mut App, String> {
    unsafe {
        let ptr = check_error(|| get_window_ptr(hwnd))
            .map_err(|err| format!("Failed to get window ptr, {err}"))?;
        let tx: &mut App = &mut *(ptr as *mut _);
        Ok(tx)
    }
}
