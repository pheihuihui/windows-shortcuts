// https://github.com/RubenZwietering/Night-Light/blob/main/Night-Light.ps1

use std::io;
use winreg::enums::RegDisposition::REG_CREATED_NEW_KEY;
use winreg::enums::RegDisposition::REG_OPENED_EXISTING_KEY;
use winreg::enums::RegType::REG_BINARY;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;
use winreg::RegValue;

const SUBKEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\Current\default$windows.data.bluelightreduction.bluelightreductionstate\windows.data.bluelightreduction.bluelightreductionstate";

pub fn enable_night_light() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let (settings, disp) = hkcu.create_subkey(SUBKEY)?;

    match disp {
        REG_CREATED_NEW_KEY => {}
        REG_OPENED_EXISTING_KEY => {
            let val = settings.get_raw_value("Data")?;
            let val18 = val.bytes[18];
            if val18 == 19u8 {
                let mut new_vec: Vec<u8> = vec![];
                for u in 0..23 {
                    new_vec.push(val.bytes[u]);
                }
                new_vec[18] = 21u8;
                new_vec.push(16u8);
                new_vec.push(0u8);
                for u in 23..41 {
                    new_vec.push(val.bytes[u]);
                }
                for u in 10..15 {
                    if new_vec[u] != 255u8 {
                        new_vec[u] += 1;
                        break;
                    }
                }
                let newval = RegValue {
                    vtype: REG_BINARY,
                    bytes: new_vec,
                };
                return settings.set_raw_value("Data", &newval);
            }
        }
    }

    Ok(())
}

pub fn disable_night_light() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let (settings, disp) = hkcu.create_subkey(SUBKEY)?;

    match disp {
        REG_CREATED_NEW_KEY => {}
        REG_OPENED_EXISTING_KEY => {
            let val = settings.get_raw_value("Data")?;
            let val18 = val.bytes[18];

            if val18 == 21u8 {
                let mut new_vec: Vec<u8> = vec![];
                for u in 0..23 {
                    new_vec.push(val.bytes[u]);
                }
                new_vec[18] = 19u8;
                for u in 25..43 {
                    new_vec.push(val.bytes[u]);
                }
                for u in 10..15 {
                    if new_vec[u] != 255u8 {
                        new_vec[u] += 1;
                        break;
                    }
                }

                let newval = RegValue {
                    vtype: REG_BINARY,
                    bytes: new_vec,
                };
                return settings.set_raw_value("Data", &newval);
            }
        }
    }

    Ok(())
}

#[allow(unused)]
pub fn reset_night_light() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let (settings, disp) = hkcu.create_subkey(SUBKEY)?;

    match disp {
        REG_CREATED_NEW_KEY => {}
        REG_OPENED_EXISTING_KEY => {
            let new_vec = vec![
                67u8, 66, 1, 0, 10, 2, 1, 0, 42, 6, 248, 203, 136, 160, 6, 42, 43, 14, 19, 67, 66,
                1, 0, 208, 10, 2, 198, 20, 131, 248, 221, 159, 138, 190, 211, 236, 1, 0, 0, 0, 0,
            ];
            let newval = RegValue {
                vtype: REG_BINARY,
                bytes: new_vec,
            };
            return settings.set_raw_value("Data", &newval);
        }
    }

    Ok(())
}

#[allow(unused)]
pub fn reset_night_light2() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let (settings, disp) = hkcu.create_subkey(SUBKEY)?;

    match disp {
        REG_CREATED_NEW_KEY => {}
        REG_OPENED_EXISTING_KEY => {
            let new_vec = vec![
                67u8, 66, 1, 0, 10, 2, 1, 0, 42, 6, 248, 203, 136, 160, 6, 42, 43, 14, 19, 67, 66,
                1, 0, 208, 10, 2, 198, 20, 131, 248, 221, 159, 138, 190, 211, 236, 1, 0, 0, 0, 0,
            ];
            let newval = RegValue {
                vtype: REG_BINARY,
                bytes: new_vec,
            };
            return settings.set_raw_value("Data", &newval);
        }
    }

    Ok(())
}