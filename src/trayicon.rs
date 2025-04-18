use crate::constants::{APP_NAME, IDM_EXIT, IDM_STARTUP, WM_USER_TRAYICON};
use crate::shortcuts::{SHORTCUTS, Shortcut};

use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::UI::Shell::{
    NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW, Shell_NotifyIconW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreateIconFromResourceEx, CreatePopupMenu, GetCursorPos, HMENU, LR_DEFAULTCOLOR,
    LookupIconIdFromDirectoryEx, MF_CHECKED, MF_STRING, MF_UNCHECKED, SetForegroundWindow,
    TPM_BOTTOMALIGN, TPM_LEFTALIGN, TrackPopupMenu,
};
use windows::core::{HSTRING, PCWSTR, w};

const ICON_BYTES: &[u8] = include_bytes!("../windows.ico");
const TEXT_STARTUP: PCWSTR = w!("Startup");
const TEXT_EXIT: PCWSTR = w!("Exit");

pub struct TrayIcon {
    data: NOTIFYICONDATAW,
}

impl TrayIcon {
    pub fn create() -> Self {
        let data = Self::create_nid();
        Self { data }
    }

    pub fn register(&mut self, hwnd: HWND) -> Result<(), String> {
        self.data.hWnd = hwnd;
        unsafe { Shell_NotifyIconW(NIM_ADD, &self.data) }
            .ok()
            .map_err(|e| format!("Fail to add trayicon, {}", e))
    }

    pub fn show(&mut self, startup: bool) -> Result<(), String> {
        let hwnd = self.data.hWnd;
        let mut cursor = POINT::default();
        unsafe {
            SetForegroundWindow(hwnd)
                .ok()
                .map_err(|e| format!("Fail to set foreground window, {}", e))?;
            GetCursorPos(&mut cursor).map_err(|e| format!("Fail to get cursor position, {}", e))?;
            let hmenu = self
                .create_menu(startup)
                .map_err(|e| format!("Fail to create menu, {}", e))?;
            let _ = TrackPopupMenu(
                hmenu,
                TPM_LEFTALIGN | TPM_BOTTOMALIGN,
                cursor.x,
                cursor.y,
                Some(0),
                hwnd,
                None,
            );
        };
        Ok(())
    }

    fn create_nid() -> NOTIFYICONDATAW {
        let offset = unsafe {
            LookupIconIdFromDirectoryEx(ICON_BYTES.as_ptr(), true, 0, 0, LR_DEFAULTCOLOR)
        };
        let icon_data = &ICON_BYTES[offset as usize..];
        let hicon =
            unsafe { CreateIconFromResourceEx(icon_data, true, 0x30000, 0, 0, LR_DEFAULTCOLOR) }
                .expect("Failed to load icon resource");
        let mut tooltip: Vec<u16> = unsafe { APP_NAME.as_wide() }.to_vec();
        tooltip.resize(128, 0);
        tooltip.pop();
        tooltip.push(0);
        let tooltip: [u16; 128] = tooltip.try_into().unwrap();
        NOTIFYICONDATAW {
            uID: WM_USER_TRAYICON,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: WM_USER_TRAYICON,
            hIcon: hicon,
            szTip: tooltip,
            ..Default::default()
        }
    }

    fn create_menu(&mut self, startup: bool) -> Result<HMENU, String> {
        let startup_flags = if startup { MF_CHECKED } else { MF_UNCHECKED };
        unsafe {
            let hmenu = CreatePopupMenu().map_err(|err| format!("Failed to create menu, {err}"))?;
            let _ = AppendMenuW(hmenu, startup_flags, IDM_STARTUP as usize, TEXT_STARTUP);

            let scs = SHORTCUTS
                .get()
                .unwrap()
                .to_vec()
                .into_iter()
                .filter(|x| x.id.is_some() && x.menu_name.is_some())
                .collect::<Vec<Shortcut>>();

            for ele in scs {
                let name = ele.menu_name.unwrap();
                let name = &HSTRING::from(name);
                let _ = AppendMenuW(hmenu, MF_STRING, ele.id.unwrap(), name);
            }

            let _ = AppendMenuW(hmenu, MF_STRING, IDM_EXIT as usize, TEXT_EXIT);
            Ok(hmenu)
        }
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        unsafe {
            let _ = Shell_NotifyIconW(NIM_DELETE, &self.data);
        };
    }
}
