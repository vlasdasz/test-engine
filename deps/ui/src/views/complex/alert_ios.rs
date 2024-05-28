#![cfg(target_os = "ios")]

use std::ffi::CString;

use dispatch::on_main;

use crate::mobile::ios::test_engine_show_alert;

static mut BUFFER: Option<CString> = None;

pub struct Alert;

impl Alert {
    pub fn show(message: impl ToString) {
        let message = message.to_string();
        on_main(move || unsafe {
            BUFFER = CString::new(message).unwrap().into();
            test_engine_show_alert(BUFFER.as_ref().unwrap().as_ptr())
        })
    }
}
