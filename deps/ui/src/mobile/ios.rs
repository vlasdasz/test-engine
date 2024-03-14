#![cfg(ios)]

extern "C" {
    pub fn ios_init_text_field();
    pub fn ios_open_keyboard();
    pub fn ios_close_keyboard();
}
