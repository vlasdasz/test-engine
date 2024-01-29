#![cfg(test)]

use std::ops::Deref;

use refs::set_current_thread_as_main;

use crate::GlFont;

#[test]
fn font() {
    set_current_thread_as_main();
    GlFont::disable_render();
    let font = GlFont::helvetica();
    dbg!(&font.deref());
}
