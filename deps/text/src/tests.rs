#![cfg(test)]

use std::ops::Deref;

use crate::Font;

#[test]
fn font() {
    Font::disable_render();
    let font = Font::helvetica();
    dbg!(&font.deref());
}
