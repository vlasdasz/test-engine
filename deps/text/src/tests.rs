#![cfg(test)]

use std::ops::Deref;

use refs::set_current_thread_as_main;

use crate::Font;

#[test]
fn font() {
    set_current_thread_as_main();
    Font::disable_render();
    let font = Font::helvetica();
    dbg!(&font.deref());
}
