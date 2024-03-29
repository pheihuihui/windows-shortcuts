use windows::core::{PCWSTR, w};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};

use crate::utils::others::to_wstring;

pub fn message_box(text: &str) {
    let text = to_wstring(text);
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(text.as_ptr() as _),
            w!("Window Switcher Error"),
            MB_OK | MB_ICONERROR,
        )
    };
}

#[macro_export]
macro_rules! alert {
    ($($arg:tt)*) => {
        $crate::utils::macros::message_box(&format!($($arg)*))
    };
}
