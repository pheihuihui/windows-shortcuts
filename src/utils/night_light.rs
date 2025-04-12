// https://github.com/RubenZwietering/Night-Light/blob/main/Night-Light.ps1

use windows::Win32::System::Registry::REG_BINARY;
use windows::Win32::System::Registry::RegSetValueExW;
use windows::core::PCWSTR;
use windows::core::w;

use crate::utils::registry::get_key;
use crate::utils::registry::get_raw_value;

const HKEY_NIGHT_LIGHT: PCWSTR = w!(
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\Current\default$windows.data.bluelightreduction.bluelightreductionstate\windows.data.bluelightreduction.bluelightreductionstate"
);
const HKEY_NAME: PCWSTR = w!("Data");

pub fn enable_night_light() -> Result<(), String> {
    let key = get_key(HKEY_NIGHT_LIGHT)?;
    let value = get_raw_value(&key.hkey, HKEY_NAME)?;

    match value {
        None => {}
        Some(arr) => {
            let val18 = arr[18];
            if val18 == 19u8 {
                let mut new_vec: Vec<u8> = vec![];
                for u in 0..23 {
                    new_vec.push(arr[u]);
                }
                new_vec[18] = 21u8;
                new_vec.push(16u8);
                new_vec.push(0u8);
                for u in 23..41 {
                    new_vec.push(arr[u]);
                }
                for u in 10..15 {
                    if new_vec[u] != 255u8 {
                        new_vec[u] += 1;
                        break;
                    }
                }
                let ret = unsafe {
                    RegSetValueExW(key.hkey, HKEY_NAME, Some(0), REG_BINARY, Some(&new_vec))
                };
                if ret.is_err() {
                    let err = format!("Fail to set reg value, {:?}", ret);
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

pub fn disable_night_light() -> Result<(), String> {
    let key = get_key(HKEY_NIGHT_LIGHT)?;
    let value = get_raw_value(&key.hkey, HKEY_NAME)?;

    match value {
        None => {}
        Some(arr) => {
            let val18 = arr[18];

            if val18 == 21u8 {
                let mut new_vec: Vec<u8> = vec![];
                for u in 0..23 {
                    new_vec.push(arr[u]);
                }
                new_vec[18] = 19u8;
                for u in 25..43 {
                    new_vec.push(arr[u]);
                }
                for u in 10..15 {
                    if new_vec[u] != 255u8 {
                        new_vec[u] += 1;
                        break;
                    }
                }
                let ret = unsafe {
                    RegSetValueExW(key.hkey, HKEY_NAME, Some(0), REG_BINARY, Some(&new_vec))
                };
                if ret.is_err() {
                    let err = format!("Fail to set reg value, {:?}", ret);
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

#[allow(unused)]
pub fn reset_night_light() -> Result<(), String> {
    let key = get_key(HKEY_NIGHT_LIGHT)?;
    let new_vec = vec![
        67u8, 66, 1, 0, 10, 2, 1, 0, 42, 6, 248, 203, 136, 160, 6, 42, 43, 14, 19, 67, 66, 1, 0,
        208, 10, 2, 198, 20, 131, 248, 221, 159, 138, 190, 211, 236, 1, 0, 0, 0, 0,
    ];
    let ret = unsafe { RegSetValueExW(key.hkey, HKEY_NAME, Some(0), REG_BINARY, Some(&new_vec)) };
    if ret.is_err() {
        let err = format!("Fail to set reg value, {:?}", ret);
        return Err(err);
    }
    Ok(())
}
