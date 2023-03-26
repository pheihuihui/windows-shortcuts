use anyhow::{anyhow, Result};
use windows::core::{Error, PCWSTR};
use windows::Win32::Foundation::{
    CloseHandle, SetLastError, BOOL, ERROR_ALREADY_EXISTS, ERROR_SUCCESS, HANDLE, HWND,
};

use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Threading::{CreateMutexW, ReleaseMutex};

use windows::Win32::UI::WindowsAndMessaging::GWL_USERDATA;

use std::path::PathBuf;

pub const BUFFER_SIZE: usize = 1024;

pub fn get_exe_folder() -> Result<PathBuf> {
    let path =
        std::env::current_exe().map_err(|err| anyhow!("Failed to get binary path, {err}"))?;
    path.parent()
        .ok_or_else(|| anyhow!("Failed to get binary folder"))
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

/// A struct representing one running instance.
pub struct SingleInstance {
    handle: Option<HANDLE>,
}

unsafe impl Send for SingleInstance {}
unsafe impl Sync for SingleInstance {}

impl SingleInstance {
    /// Returns a new SingleInstance object.
    pub fn create(name: &str) -> Result<Self> {
        let name = to_wstring(name);
        let handle = unsafe { CreateMutexW(None, BOOL(1), PCWSTR(name.as_ptr())) }
            .map_err(|err| anyhow!("Fail to setup single instance, {err}"))?;
        let handle =
            if windows::core::Error::from_win32().code() == ERROR_ALREADY_EXISTS.to_hresult() {
                None
            } else {
                Some(handle)
            };
        Ok(SingleInstance { handle })
    }

    /// Returns whether this instance is single.
    pub fn is_single(&self) -> bool {
        self.handle.is_some()
    }
}

impl Drop for SingleInstance {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            unsafe {
                ReleaseMutex(handle);
                CloseHandle(handle);
            }
        }
    }
}

pub fn parse_mac_addr(mac: &str) -> Result<[u8; 6], &str> {
    let arr = mac.split(":").collect::<Vec<&str>>();
    let mut res: [u8; 6] = [0; 6];
    if arr.len() != 6 {
        return Err("failed 1");
    }
    for u in 0..6 {
        match u8::from_str_radix(arr[u], 16) {
            Ok(val) => {
                res[u] = val;
            }
            Err(_) => {
                return Err("failed 2");
            }
        }
    }
    Ok(res)
}
