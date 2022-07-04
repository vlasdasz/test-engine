use gm::{
    flat::{Point, PointsPath},
    Color,
};
use rtools::{Event, Rglica, ToRglica};

use crate::{
    complex::DrawingView,
    impl_view, view,
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks, ViewData, ViewTouch,
};

const SIZE: f32 = 140.0;
const OUTLINE_WIDTH: f32 = 10.0;
const STICK_VIEW_SIZE: f32 = SIZE / 2.0;

#[view]
#[derive(Default, Debug)]
pub struct AnalogStickView {
    direction_stick: Rglica<DrawingView>,
    background:      Rglica<DrawingView>,
    pub on_change:   Event<Point>,
    pub flaccid:     bool,
}

impl_view!(AnalogStickView);

impl AnalogStickView {
    fn on_touch_moved(&mut self, touch: &Point) {
        let max_lenght = self.frame().size.height / 2.0;
        let center = self.frame().size.center();

        let vector = (touch - &center).trimmed(max_lenght);

        let frame = *self.frame();

        self.direction_stick.set_center(vector + frame.size.center());

        self.on_change.trigger(vector * 0.1);
    }
}

impl ViewCallbacks for AnalogStickView {
    fn setup(&mut self) {
        self.set_frame((SIZE, SIZE)).set_color(Color::CLEAR);

        self.on_touch().set(self, |this, touch| {
            if touch.is_ended() {
                if this.flaccid {
                    return;
                }
                let frame = *this.frame();
                this.direction_stick.set_center(frame.size.center());
                this.on_change.trigger(Point::default());
            } else {
                this.on_touch_moved(&touch.position);
            }
        });

        self.background = self.add_view();
        self.background.set_frame((SIZE, SIZE)).set_color(Color::CLEAR);

        let frame = *self.frame();
        self.background.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width),
            Color::BLACK,
        );

        self.background.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width - OUTLINE_WIDTH),
            Color::WHITE,
        );

        self.direction_stick = self.add_view();
        let mut direction_stick = self.direction_stick;

        direction_stick
            .set_frame((STICK_VIEW_SIZE, STICK_VIEW_SIZE))
            .set_center(self.frame().size.center())
            .set_color(Color::CLEAR);

        let stick_center = direction_stick.frame().size.center();

        direction_stick
            .add_path(
                PointsPath::circle_with(stick_center, STICK_VIEW_SIZE),
                Color::BLACK,
            )
            .add_path(
                PointsPath::circle_with(stick_center, STICK_VIEW_SIZE - OUTLINE_WIDTH),
                Color::LIGHT_GRAY,
            );
    }
}
