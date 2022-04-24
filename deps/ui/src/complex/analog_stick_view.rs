use gm::{
    flat::{Point, PointsPath},
    Color,
};
use rtools::{Event, Rglica};

use crate::{
    complex::DrawingView,
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewTouch,
};

const SIZE: f32 = 140.0;
const OUTLINE_WIDTH: f32 = 10.0;
const STICK_VIEW_SIZE: f32 = SIZE / 2.0;

#[derive(Default, Debug)]
pub struct AnalogStickView {
    base:            ViewBase,
    direction_stick: Rglica<DrawingView>,
    background:      Rglica<DrawingView>,
    pub on_change:   Event<Point>,
    pub flaccid:     bool,
}

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

impl View for AnalogStickView {
    fn setup(&mut self) {
        self.set_frame((SIZE, SIZE));

        self.on_touch().set(self, |touch, this| {
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
            PointsPath::circle_with(frame.size.center(), frame.size.width),
            Color::BLACK,
        );

        self.background.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width - OUTLINE_WIDTH),
            Color::WHITE,
        );

        self.direction_stick = self.add_view();
        let mut direction_stick = self.direction_stick;

        direction_stick.set_frame((STICK_VIEW_SIZE, STICK_VIEW_SIZE));

        direction_stick.set_center(self.frame().size.center());

        let stick_center = direction_stick.frame().size.center();

        direction_stick.add_path(
            PointsPath::circle_with(stick_center, STICK_VIEW_SIZE),
            Color::BLACK,
        );

        direction_stick.add_path(
            PointsPath::circle_with(stick_center, STICK_VIEW_SIZE - OUTLINE_WIDTH),
            Color::LIGHT_GRAY,
        );
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
