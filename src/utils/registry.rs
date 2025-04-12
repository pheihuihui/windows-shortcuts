use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_ALL_ACCESS, REG_BINARY, REG_VALUE_TYPE, RRF_RT_REG_BINARY,
    RRF_RT_REG_SZ, RegCloseKey, RegGetValueW, RegOpenKeyExW,
};
use windows::core::PCWSTR;

use super::others::BUFFER_SIZE;

pub struct WrapHKey {
    pub hkey: HKEY,
}

impl Drop for WrapHKey {
    fn drop(&mut self) {
        let _ = unsafe { RegCloseKey(self.hkey) };
    }
}

pub fn get_key(name: PCWSTR) -> Result<WrapHKey, String> {
    let mut hkey = HKEY::default();
    let ret = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            name,
            Some(0),
            KEY_ALL_ACCESS,
            &mut hkey as *mut _,
        )
    };
    if ret.is_err() {
        let err = format!("Fail to open reg key, {:?}", ret);
        return Err(err);
    }
    Ok(WrapHKey { hkey })
}

pub fn get_value(hkey: &HKEY, val_name: PCWSTR) -> Result<Option<Vec<u16>>, String> {
    let mut buffer: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut size = (BUFFER_SIZE * std::mem::size_of_val(&buffer[0])) as u32;
    let mut kind: REG_VALUE_TYPE = Default::default();
    let ret = unsafe {
        RegGetValueW(
            *hkey,
            None,
            val_name,
            RRF_RT_REG_SZ,
            Some(&mut kind),
            Some(buffer.as_mut_ptr() as *mut _),
            Some(&mut size),
        )
    };
    if ret.is_err() {
        if ret == ERROR_FILE_NOT_FOUND.into() {
            return Ok(None);
        }
        let err = format!("Fail to get reg value, {:?}", ret);
        return Err(err);
    }
    let len = (size as usize - 1) / 2;
    Ok(Some(buffer[..len].to_vec()))
}

pub fn get_raw_value(hkey: &HKEY, val_name: PCWSTR) -> Result<Option<Vec<u8>>, String> {
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut size = (BUFFER_SIZE * std::mem::size_of_val(&buffer[0])) as u32;
    let mut kind: REG_VALUE_TYPE = REG_BINARY;
    let ret = unsafe {
        RegGetValueW(
            *hkey,
            None,
            val_name,
            RRF_RT_REG_BINARY,
            Some(&mut kind),
            Some(buffer.as_mut_ptr() as *mut _),
            Some(&mut size),
        )
    };
    if ret.is_err() {
        if ret == ERROR_FILE_NOT_FOUND.into() {
            return Ok(None);
        }
        let err = format!("Fail to get reg value, {:?}", ret);
        return Err(err);
    }
    Ok(Some(buffer[..size as usize].to_vec()))
}
