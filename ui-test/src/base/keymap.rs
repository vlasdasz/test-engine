use anyhow::Result;
use test_engine::{
    refs::Own,
    ui::{UI, UIManager, view},
    ui_test::inject_key,
    wait_for_next_frame,
};

#[view]
struct Keymap {}

pub async fn test_keymap() -> Result<()> {
    let view = UI::init_test_view::<Keymap>().await;

    let presses = Own::new(0);
    let mut presses = presses.weak();

    assert_eq!(*presses, 0);

    UIManager::keymap().add(view, 'g', move || {
        *presses += 1;
    });

    assert_eq!(*presses, 0);

    inject_key('a').await;
    assert_eq!(*presses, 0);

    inject_key('b').await;
    assert_eq!(*presses, 0);

    inject_key('c').await;
    assert_eq!(*presses, 0);

    inject_key('g').await;
    assert_eq!(*presses, 1);

    inject_key('g').await;
    assert_eq!(*presses, 2);

    UI::init_test_view::<Keymap>().await;
    wait_for_next_frame().await;

    inject_key('g').await;
    assert_eq!(*presses, 2);

    inject_key('g').await;
    assert_eq!(*presses, 2);

    Ok(())
}
