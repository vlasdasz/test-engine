use std::{fmt::Display, process::ExitCode};

use glfw::MouseButtonLeft;
use log::error;
use serde::de::DeserializeOwned;
use test_engine::{from_main, gl_wrapper::system_events::SystemEvents, on_main, ui_layer::UILayer};
use tokio::{spawn, sync::mpsc::channel};
use ui::{input::UIEvents, refs::ToOwn, Touch};

use crate::view_tests::state::{clear_state, get_state};

pub mod state;

#[allow(dead_code)]
pub fn test_combinations<const A: usize, Val>(comb: [(&'static str, Val); A])
where Val: Display + PartialEq + DeserializeOwned + Default + Send + 'static {
    spawn(async move {
        for comb in comb {
            clear_state();

            let touches = Touch::vec_from_str(comb.0);

            for touch in touches {
                inject_touch(touch).await;
            }

            if get_state::<Val>() != comb.1 {
                error!(
                    "Failed state for: {}Expected: {} got: {}",
                    comb.0,
                    comb.1,
                    get_state::<Val>()
                );
                SystemEvents::terminate(ExitCode::FAILURE);
            }
        }
        SystemEvents::terminate(ExitCode::SUCCESS);
    });
}

#[allow(dead_code)]
pub async fn inject_touch(touch: impl Into<Touch>) {
    let touch = touch.into();
    from_main(move || {
        let events = SystemEvents::get();
        events.cursor_moved.trigger(touch.position);
        events.mouse_click.trigger((MouseButtonLeft, touch.event.glfw_action()));
    })
    .await;
}

#[allow(dead_code)]
pub async fn record_touches() {
    let touches = Vec::<Touch>::new().to_own();
    let mut touches = touches.weak();

    let (s, mut r) = channel::<()>(1);

    on_main(move || {
        UIEvents::get().on_touch.val(move |touch| {
            if touches.is_null() {
                return;
            }

            if touch.is_moved() {
                return;
            }

            touches.push(touch);
        });

        UILayer::keymap().add('a', move || {
            _ = s.try_send(());
        });
    });

    r.recv().await.unwrap();

    println!("{}", Touch::str_from_vec(touches.to_vec()));

    SystemEvents::terminate(ExitCode::SUCCESS);
}
