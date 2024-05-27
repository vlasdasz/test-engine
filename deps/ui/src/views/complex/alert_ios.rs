#![cfg(target_os = "ios")]

use crate::mobile::ios::test_engine_show_alert;

pub struct Alert;

impl Alert {
    pub fn show(message: impl ToString) {
        let message = message.to_string();
        unsafe { test_engine_show_alert(message.as_ptr().cast()) };
    }
}
