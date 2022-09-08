use gm::{
    flat::{Point, PointsPath},
    Color,
};
use rtools::{Event, Rglica, ToRglica};

use crate::{
    complex::{DrawMode, DrawingView},
    view,
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks, ViewTouch,
};

const SIZE: f32 = 80.0;
const OUTLINE_WIDTH: f32 = 5.0;
const STICK_VIEW_SIZE: f32 = SIZE / 2.0;
const PRECISION: u16 = 50;

#[view]
#[derive(Default, Debug)]
pub struct AnalogStickView {
    direction_stick: Rglica<DrawingView>,
    background:      Rglica<DrawingView>,
    pub on_change:   Event<Point>,
    pub flaccid:     bool,
}

impl AnalogStickView {
    fn on_touch_moved(&mut self, touch: &Point) {
        let max_length = self.frame().size.height / 2.0;
        let center = self.frame().size.center();

        let vector = (touch - &center).trimmed(max_length);

        let frame = *self.frame();

        self.direction_stick.set_center(vector + frame.size.center());

        self.on_change.trigger(vector * 0.1);
    }
}

impl ViewCallbacks for AnalogStickView {
    fn setup(&mut self) {
        self.set_frame((SIZE, SIZE));

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
        self.background.set_frame((SIZE, SIZE));

        let frame = *self.frame();
        self.background.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width / 2.0, PRECISION),
            &Color::BLACK,
            DrawMode::Fill,
        );

        self.background.add_path(
            PointsPath::circle_with(
                frame.size.center(),
                (frame.size.width - OUTLINE_WIDTH) / 2.0,
                PRECISION,
            ),
            &Color::WHITE,
            DrawMode::Fill,
        );

        self.direction_stick = self.add_view();
        let mut direction_stick = self.direction_stick;

        direction_stick
            .set_frame((STICK_VIEW_SIZE, STICK_VIEW_SIZE))
            .set_center(self.frame().size.center());

        let stick_center = direction_stick.frame().size.center();

        direction_stick
            .add_path(
                PointsPath::circle_with(stick_center, STICK_VIEW_SIZE / 2.0, PRECISION),
                &Color::BLACK,
                DrawMode::Fill,
            )
            .add_path(
                PointsPath::circle_with(stick_center, (STICK_VIEW_SIZE - OUTLINE_WIDTH) / 2.0, PRECISION),
                &Color::LIGHT_GRAY,
                DrawMode::Fill,
            );
    }
}
