use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::UI::WindowsAndMessaging::GWL_USERDATA;

use std::path::PathBuf;

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
