pub mod helpers;
pub mod state;

use std::{
    fmt::Display,
    ops::Deref,
    sync::{Arc, Mutex},
};

use anyhow::{Result, bail};
pub use helpers::*;
use log::{error, warn};
use refs::Own;
use serde::de::DeserializeOwned;
pub use state::*;
use tokio::sync::mpsc::channel;
use window::Window;

use crate::{
    App, from_main,
    gm::{LossyConvert, ToF32},
    on_main,
    ui::{Input, Touch, U8Color, UIEvents, UIManager},
    wait_for_next_frame,
};

pub async fn test_combinations<const A: usize, Val>(comb: [(&'static str, Val); A]) -> Result<()>
where Val: Display + PartialEq + DeserializeOwned + Default + Send + 'static {
    for comb in comb {
        clear_state();

        let touches = Touch::vec_from_str(comb.0);

        for touch in touches {
            from_main(move || {
                inject_touch(touch);
            })
            .await;
        }

        if get_state::<Val>() != comb.1 {
            error!(
                "Failed state for: {} Expected: {} got: {}",
                comb.0,
                comb.1,
                get_state::<Val>()
            );
            bail!("UI test failed")
        }
    }
    Ok(())
}

fn inject_touch(touch: impl Into<Touch> + Send + Copy + 'static) {
    Input::process_touch_event(touch.into());
}

#[allow(dead_code)]
pub async fn inject_scroll(scroll: impl ToF32) {
    from_main(move || {
        UIManager::trigger_scroll((0, scroll).into());
    })
    .await;
}

pub async fn inject_touches(data: impl ToString + Send + 'static) {
    from_main(move || {
        for touch in Touch::vec_from_str(&data.to_string()) {
            inject_touch(touch);
        }
    })
    .await;
}

pub async fn inject_touches_delayed(data: &str) {
    for touch in Touch::vec_from_str(data) {
        wait_for_next_frame().await;
        from_main(move || {
            inject_touch(touch);
        })
        .await;
        wait_for_next_frame().await;
    }
}

pub async fn inject_keys(s: impl ToString) {
    let s = s.to_string();
    for ch in s.chars() {
        inject_key(ch).await;
    }
}

pub async fn inject_key(key: char) {
    from_main(move || Input::on_char(key)).await;
}

#[allow(dead_code)]
pub async fn record_touches() {
    record_touches_internal(true).await;
}

#[allow(dead_code)]
pub async fn record_moved_touches() {
    record_touches_internal(false).await;
}

async fn record_touches_internal(skip_moved: bool) {
    let touches: Own<_> = Vec::<Touch>::new().into();
    let mut touches = touches.weak();

    let (s, mut r) = channel::<()>(1);

    let moved_record_skip = 10;

    let moved_counter = Arc::new(Mutex::new(0));

    on_main(move || {
        UIEvents::on_touch().val(move |touch| {
            if touch.is_moved() {
                let mut counter = moved_counter.lock().unwrap();
                *counter += 1;
                if *counter == moved_record_skip {
                    *counter = 0;
                } else {
                    return;
                }
            }

            if skip_moved && touch.is_moved() {
                return;
            }

            touches.push(touch);
        });

        UIManager::keymap().add(UIManager::root_view_weak(), 'a', move || {
            _ = s.try_send(());
        });
    });

    if r.recv().await.is_none() {
        warn!("Failed to receive record_touches result");
    }

    from_main(|| {
        UIEvents::on_touch().remove_subscribers();
    })
    .await;

    println!(
        r#"
        inject_touches(
        "
{}
        ",
    )
        .await;
    "#,
        Touch::str_from_vec(touches.to_vec()),
    );
}

#[allow(dead_code)]
pub async fn record_ui_test() {
    loop {
        Window::set_title("Recording touches");
        record_touches().await;
        Window::set_title("Recording colors");
        record_colors().await.unwrap();
    }
}

#[allow(dead_code)]
pub async fn record_colors() -> Result<()> {
    let touch_lock = Touch::lock();

    let screenshot = App::take_screenshot().await?;

    let touches: Own<_> = Vec::<(Touch, U8Color)>::new().into();
    let mut touches = touches.weak();

    let (s, mut r) = channel::<()>(1);

    on_main(move || {
        UIEvents::on_debug_touch().val(move |touch| {
            if !touch.is_began() {
                return;
            }

            touches.push((touch, screenshot.get_pixel(touch.position)));
        });

        UIManager::keymap().add(UIManager::root_view_weak(), 'a', move || {
            _ = s.try_send(());
        });
    });

    if r.recv().await.is_none() {
        warn!("Failed to receive record_touches_with_colors result");
    }

    on_main(|| {
        UIEvents::on_touch().remove_subscribers();
        UIEvents::on_debug_touch().remove_subscribers();
    });

    println!("check_colors( r#\"");

    for (touch, color) in touches.deref() {
        let x: u32 = touch.position.x.lossy_convert();
        let y: u32 = touch.position.y.lossy_convert();
        println!(
            "            {:>4} {:>4} - {:>3} {:>3} {:>3}",
            x, y, color.r, color.g, color.b
        );
    }

    println!("        \"#");
    println!(").await?;");

    drop(touch_lock);

    Ok(())
}
