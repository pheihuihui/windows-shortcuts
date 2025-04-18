use windows::Win32::System::Registry::{REG_SZ, RegDeleteValueW, RegSetValueExW};
use windows::core::{PCWSTR, w};

use crate::utils::others::get_exe_path;
use crate::utils::registry::{get_key, get_value};

const HKEY_RUN: PCWSTR = w!(r"Software\Microsoft\Windows\CurrentVersion\Run");
const HKEY_NAME: PCWSTR = w!("Windows Shortcuts");

#[derive(Default)]
pub struct Startup {
    pub is_enable: bool,
}

impl Startup {
    pub fn init() -> Result<Self, String> {
        let enable = Self::detect()?;
        Ok(Self { is_enable: enable })
    }

    pub fn toggle(&mut self) -> Result<(), String> {
        let is_enable = self.is_enable;
        if is_enable {
            Self::disable()?;
            self.is_enable = false;
        } else {
            Self::enable()?;
            self.is_enable = true;
        }
        Ok(())
    }

    fn detect() -> Result<bool, String> {
        let key = get_key(HKEY_RUN)?;
        let value = match get_value(&key.hkey, HKEY_NAME)? {
            Some(value) => value,
            None => return Ok(false),
        };
        let path = get_exe_path();
        Ok(value == path)
    }

    fn enable() -> Result<(), String> {
        let key = get_key(HKEY_RUN)?;
        let path = get_exe_path();
        let path_u8 = unsafe { path.align_to::<u8>().1 };
        let ret = unsafe { RegSetValueExW(key.hkey, HKEY_NAME, Some(0), REG_SZ, Some(path_u8)) };
        if ret.is_err() {
            let err = format!("Fail to write reg value, {:?}", ret);
            return Err(err);
        }
        Ok(())
    }

    fn disable() -> Result<(), String> {
        let key = get_key(HKEY_RUN)?;
        let ret = unsafe { RegDeleteValueW(key.hkey, HKEY_NAME) };
        if ret.is_err() {
            let err = format!("Fail to delele reg value, {:?}", ret);
            return Err(err);
        }
        Ok(())
    }
}
