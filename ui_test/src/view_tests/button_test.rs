use std::process::ExitCode;

use glfw::MouseButtonLeft;
use log::error;
use test_engine::{
    from_main, gl_wrapper::system_events::SystemEvents, gm::flat::IntSize, on_main, ui_layer::UILayer,
};
use tokio::{spawn, sync::mpsc::channel};
use ui::{
    input::UIEvents,
    refs::{Own, Weak},
    view, SubView, Touch, ViewSetup, ViewTest,
};
use ui_views::Button;

use crate::view_tests::state::{clear_state, get_state, increment_state};

#[view]
struct ButtonTestView {
    button: SubView<Button>,
}

impl ViewSetup for ButtonTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.place.back().size(100, 50).center();
        self.button.set_text("Button text");

        self.button.on_tap.sub(|| {
            increment_state();
        });
    }
}

impl ViewTest for ButtonTestView {
    fn test_size() -> IntSize
    where Self: Sized {
        (200, 100).into()
    }
}

fn test_combinations<const A: usize>(comb: [(&'static str, u32); A]) {
    spawn(async move {
        for comb in comb {
            clear_state();

            let touches = Touch::vec_from_str(comb.0);

            for touch in touches {
                inject_touch(touch).await;
            }

            if get_state() != comb.1 {
                error!(
                    "Failed state for: {}Expected: {} got: {}",
                    comb.0,
                    comb.1,
                    get_state()
                );
                SystemEvents::terminate(ExitCode::FAILURE);
            }
        }
        SystemEvents::terminate(ExitCode::SUCCESS);
    });
}

async fn inject_touch(touch: Touch) {
    from_main(move || {
        let events = SystemEvents::get();
        events.cursor_moved.trigger(touch.position);
        events.mouse_click.trigger((MouseButtonLeft, touch.event.glfw_action()));
    })
    .await;
}

async fn record_touches() {
    let touches = Own::new(vec![]);
    let mut touches = touches.weak();

    let (s, mut r) = channel::<()>(1);

    on_main(move || {
        UIEvents::get().on_touch.val(move |touch| {
            touches.push(touch);
        });

        UILayer::keymap().add('a', move || {
            s.try_send(()).unwrap();
        });
    });

    r.recv().await.unwrap();

    dbg!(&touches);
}

pub fn test_button_view() -> ExitCode {
    test_engine::ViewApp::<ButtonTestView>::start_with_actor(async {
        return record_touches().await;
        test_combinations([
            ("0 0 ↓", 0),
            ("0 0 ↑", 0),
            // Begin inside end outside
            ("100 50 ↓", 0),
            ("  0 50 ↑", 0),
            // Begin inside end outside
            ("100 50 ↓", 0),
            ("  0 50 ↑", 0),
            // Simple tap
            (
                r#"
                100 50 ↓
                100 50 ↑
            "#,
                1,
            ),
            // Simple tap
            (
                r#"
                 90 50 ↓
                110 50 ↑
            "#,
                1,
            ),
            // Double release
            (
                r#"
                 90 50 ↓
                110 50 ↑
                110 50 ↑
            "#,
                1,
            ),
            // Outside then inside
            (
                r#"
                  0 50 ↓
                110 50 ↑
            "#,
                0,
            ),
        ]);
    })
}
