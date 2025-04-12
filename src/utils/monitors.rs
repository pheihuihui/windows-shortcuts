use windows::Win32::Devices::Display::{
    SDC_APPLY, SDC_TOPOLOGY_EXTERNAL, SDC_TOPOLOGY_INTERNAL, SetDisplayConfig,
};

pub fn set_external_display() -> i32 {
    let flags = SDC_TOPOLOGY_EXTERNAL | SDC_APPLY;
    let result = unsafe { SetDisplayConfig(None, None, flags) };
    result
}

pub fn set_internal_display() -> i32 {
    let flags = SDC_TOPOLOGY_INTERNAL | SDC_APPLY;
    let result = unsafe { SetDisplayConfig(None, None, flags) };
    result
}
