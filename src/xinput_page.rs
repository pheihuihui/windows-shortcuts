extern crate native_windows_gui as nwg;
use nwg::NativeUi;



#[derive(Default)]
pub struct XinputPage {
    window: nwg::Window,
    name_edit: nwg::TextBox,
    hello_button: nwg::Button,
}

impl XinputPage {
    fn say_hello(&self) {
        nwg::modal_info_message(
            &self.window,
            "Hello",
            &format!("Hello {}", self.name_edit.text()),
        );
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(
            &self.window,
            "Goodbye",
            &format!("Goodbye {}", self.name_edit.text()),
        );
        nwg::stop_thread_dispatch();
    }
}

