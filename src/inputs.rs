use std::mem::size_of;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VK_MENU, VK_TAB,
};

pub fn switch_windows() {
    let input_0 = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_MENU,
                wScan: 1,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let input_1 = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_TAB,
                wScan: 1,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 10,
                dwExtraInfo: 0,
            },
        },
    };

    let input_2 = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_TAB,
                wScan: 1,
                dwFlags: KEYEVENTF_KEYUP,
                time: 20,
                dwExtraInfo: 0,
            },
        },
    };

    let input_3 = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_MENU,
                wScan: 1,
                dwFlags: KEYEVENTF_KEYUP,
                time: 30,
                dwExtraInfo: 0,
            },
        },
    };

    let inputs = [input_0, input_1, input_2, input_3];
    unsafe {
        let res = SendInput(&inputs, size_of::<INPUT>() as i32);
        println!("{:?}", res);
    }
}
