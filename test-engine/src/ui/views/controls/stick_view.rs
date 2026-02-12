use gm::flat::Point;
use refs::Weak;
use ui::{ImageView, Setup, Touch, UIImages, ViewData, ViewFrame, ViewTouch};
use ui_proc::view;
use vents::Event;

use crate as test_engine;

#[view]
pub struct StickView {
    pub on_change: Event<Point>,
    pub flaccid:   bool,

    #[init]
    background:      ImageView,
    direction_stick: ImageView,
}

impl StickView {
    fn on_touch_moved(&mut self, touch: Point) {
        let max_length = self.frame().size.height / 2.0;
        let center = self.frame().size.center();

        let vector = (touch - center).trimmed(max_length);

        let frame = *self.frame();

        self.direction_stick.set_center(vector + frame.size.center());

        self.on_change.trigger(vector * 0.1);
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            if self.flaccid {
                return;
            }
            let frame = *self.frame();
            self.direction_stick.set_center(frame.size.center());
            self.on_change.trigger(Point::default());
        } else {
            self.on_touch_moved(touch.position);
        }
    }
}

impl Setup for StickView {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.touch().all.val(move |touch| {
            self.direction_stick.place().clear().relative_size(self, 0.5);
            self.on_touch(&touch);
        });

        self.background.set_image(UIImages::joystick());
        self.direction_stick.set_image(UIImages::handle());

        self.background.place().back();

        let _center = self.frame().size.center();

        self.direction_stick.place().relative_size(self, 0.5).center();

        // self.direction_stick
        //     .set_frame((0, 0, STICK_VIEW_SIZE, STICK_VIEW_SIZE))
        //     .set_center(center);
        //
        // self.chan
    }
}

mod test {
    use anyhow::Result;
    use refs::Weak;
    use ui::{Setup, ViewData, ViewTest, view_test};

    use crate as test_engine;
    use crate::{ui::StickView, ui_test::record_ui_test};

    #[view_test]
    struct StickViewTest {
        #[init]
        stick: StickView,
    }

    impl Setup for StickViewTest {
        fn setup(self: Weak<Self>) {
            self.stick.place().back();
        }
    }

    impl ViewTest for StickViewTest {
        fn perform_test(_view: Weak<Self>) -> Result<()> {
            record_ui_test();

            Ok(())
        }
    }
}
