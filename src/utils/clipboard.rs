use windows::Win32::System::DataExchange::{CloseClipboard, EmptyClipboard, OpenClipboard};

pub fn clear_clipboard() {
    unsafe {
        let _ = OpenClipboard(None);
        let _ = EmptyClipboard();
        let _ = CloseClipboard();
    }
}
