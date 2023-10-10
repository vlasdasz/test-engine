use glfw::{Action, MouseButtonLeft};
use test_engine::{
    from_main,
    gl_wrapper::system_events::SystemEvents,
    gm::flat::{Point, Size},
};
use tokio::spawn;
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
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
    fn test_size() -> Size
    where Self: Sized {
        (200, 100).into()
    }
}

fn test_combinations<const A: usize>(comb: [(&'static [(impl Into<Point> + Copy + Sync, Action)], u32); A]) {
    spawn(async move {
        for comb in comb {
            clear_state();
            for touch in comb.0 {
                inject_touch(touch.0.into(), touch.1).await;
            }
            assert_eq!(get_state(), comb.1);
        }
        SystemEvents::terminate();
    });
}

async fn inject_touch(pos: Point, action: Action) {
    from_main(move || {
        let events = SystemEvents::get();
        events.cursor_moved.trigger(pos);
        events.mouse_click.trigger((MouseButtonLeft, action));
    })
    .await;
}

pub fn test_button_view() {
    test_engine::ViewApp::<ButtonTestView>::start_with_actor(|| {
        test_combinations([
            (&[((0, 0), Action::Press)], 0),
            (&[((0, 0), Action::Release)], 0),
            // Begin inside end outside
            (&[((100, 50), Action::Press)], 0),
            (&[((0, 50), Action::Release)], 0),
            // Begin inside end outside
            (&[((100, 50), Action::Press)], 0),
            (&[((0, 50), Action::Release)], 0),
            // Simple tap
            (&[((100, 50), Action::Press), ((100, 50), Action::Release)], 1),
            // Simple tap
            (&[((90, 50), Action::Press), ((110, 50), Action::Release)], 1),
            // Double release
            (
                &[
                    ((90, 50), Action::Press),
                    ((110, 50), Action::Release),
                    ((110, 50), Action::Release),
                ],
                1,
            ),
            // Outside then inside
            (&[((0, 50), Action::Press), ((110, 50), Action::Release)], 0),
        ]);
    });
}
