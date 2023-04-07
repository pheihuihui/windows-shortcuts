use windows::core::Error;
use windows::Win32::Foundation::{SetLastError, ERROR_SUCCESS, HANDLE, HWND};

use windows::Win32::System::LibraryLoader::GetModuleFileNameW;

use windows::Win32::UI::WindowsAndMessaging::GWL_USERDATA;

use std::path::PathBuf;
use std::thread;
use std::time;

use crate::constants::APP_CONFIG;
use crate::utils::adb::{
    capture_screen_adb, connect_tv_adb, sleep_tv_adb, switch_to_home, switch_to_port_4,
    wakeup_tv_adb,
};
use crate::utils::magic_packet::MagicPacket;
use crate::utils::monitors::{set_external_display, set_internal_display};
use crate::utils::night_light::{disable_night_light, enable_night_light};

pub const BUFFER_SIZE: usize = 1024;

pub fn get_exe_folder() -> Result<PathBuf, String> {
    let path =
        std::env::current_exe().map_err(|err| format!("Failed to get binary path, {err}"))?;
    path.parent()
        .ok_or_else(|| format!("Failed to get binary folder"))
        .map(|v| v.to_path_buf())
}

pub fn get_exe_path() -> Vec<u16> {
    let mut path = vec![0u16; BUFFER_SIZE];
    let size = unsafe { GetModuleFileNameW(None, &mut path) } as usize;
    path[..size].to_vec()
}

#[cfg(target_arch = "x86_64")]
pub fn get_window_ptr(hwnd: HWND) -> isize {
    unsafe { windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(hwnd, GWL_USERDATA) }
}

#[cfg(target_arch = "x86_64")]
pub fn set_window_ptr(hwnd: HWND, ptr: isize) -> isize {
    unsafe { windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW(hwnd, GWL_USERDATA, ptr) }
}

#[inline]
/// Use to wrap fallible Win32 functions.
/// First calls SetLastError(0).
/// And then after checks GetLastError().
/// Useful when the return value doesn't reliably indicate failure.
pub fn check_error<F, R>(mut f: F) -> windows::core::Result<R>
where
    F: FnMut() -> R,
{
    unsafe {
        SetLastError(ERROR_SUCCESS);
        let result = f();
        let error = Error::from_win32();
        if error == Error::OK {
            Ok(result)
        } else {
            Err(error)
        }
    }
}

pub trait CheckError: Sized {
    fn check_error(self) -> windows::core::Result<Self>;
}

impl CheckError for HANDLE {
    fn check_error(self) -> windows::core::Result<Self> {
        if self.is_invalid() {
            Err(Error::from_win32())
        } else {
            Ok(self)
        }
    }
}

impl CheckError for HWND {
    fn check_error(self) -> windows::core::Result<Self> {
        // If the function fails, the return value is NULL.
        if self.0 == 0 {
            Err(Error::from_win32())
        } else {
            Ok(self)
        }
    }
}

impl CheckError for u16 {
    fn check_error(self) -> windows::core::Result<Self> {
        // If the function fails, the return value is zero
        if self == 0 {
            Err(Error::from_win32())
        } else {
            Ok(self)
        }
    }
}

pub fn to_wstring(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect::<Vec<u16>>()
}

pub fn parse_mac_addr(mac: &str) -> Result<[u8; 6], String> {
    let arr = mac.split(":").collect::<Vec<&str>>();
    let mut res: [u8; 6] = [0; 6];
    if arr.len() != 6 {
        return Err("failed 1".to_string());
    }
    for u in 0..6 {
        match u8::from_str_radix(arr[u], 16) {
            Ok(val) => {
                res[u] = val;
            }
            Err(_) => {
                return Err("failed 2".to_string());
            }
        }
    }
    Ok(res)
}

pub fn parse_ip_addr(mac: &str) -> Result<[u8; 4], String> {
    let arr = mac.split(".").collect::<Vec<&str>>();
    let mut res: [u8; 4] = [0; 4];
    if arr.len() != 4 {
        return Err("failed 1".to_string());
    }
    for u in 0..4 {
        match u8::from_str_radix(arr[u], 10) {
            Ok(val) => {
                res[u] = val;
            }
            Err(_) => {
                return Err("failed 2".to_string());
            }
        }
    }
    Ok(res)
}

pub fn switch_to_tv() {
    let mac = APP_CONFIG.tv_mac_addr;
    let ip = &APP_CONFIG.tv_ip_addr;
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
}

pub fn switch_to_monitor() {
    let ip = &APP_CONFIG.tv_ip_addr;
    thread::spawn(move || {
        connect_tv_adb(&ip);
        thread::sleep(time::Duration::from_millis(200));
        switch_to_home();
        thread::sleep(time::Duration::from_millis(200));
        enable_night_light().unwrap();
        set_internal_display();
        sleep_tv_adb();
    });
}

pub fn capture_screen() {
    let ip = APP_CONFIG.tv_ip_addr.to_owned();
    let dir = APP_CONFIG.screen_dir.to_owned();
    connect_tv_adb(&ip);
    capture_screen_adb(&dir);
}
