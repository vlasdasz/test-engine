use anyhow::Result;
use test_engine::{
    dispatch::wait_for_next_frame,
    refs::Own,
    ui::{UIDrawer, UIManager, view},
    ui_test::inject_key,
};

#[view]
struct Keymap {}

pub async fn test_keymap() -> Result<()> {
    let view = UIDrawer::init_test_view::<Keymap>();

    let presses = Own::new(0);
    let mut presses = presses.weak();

    assert_eq!(*presses, 0);

    UIManager::keymap().add(view, 'g', move || {
        *presses += 1;
    });

    assert_eq!(*presses, 0);

    inject_key('a');
    assert_eq!(*presses, 0);

    inject_key('b');
    assert_eq!(*presses, 0);

    inject_key('c');
    assert_eq!(*presses, 0);

    inject_key('g');
    assert_eq!(*presses, 1);

    inject_key('g');
    assert_eq!(*presses, 2);

    UIDrawer::init_test_view::<Keymap>();
    wait_for_next_frame();

    inject_key('g');
    assert_eq!(*presses, 2);

    inject_key('g');
    assert_eq!(*presses, 2);

    Ok(())
}
