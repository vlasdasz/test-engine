use gm::{
    flat::{Point, PointsPath},
    Color,
};
use rtools::{rglica::ToRglica, Boxed, Event, Rglica};

use crate::{complex::DrawingView, view_base::ViewBase, Touch, View};

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

        self.direction_stick
            .frame_mut()
            .set_center(vector + frame.size.center());

        self.on_change.trigger(vector * 0.1);
    }
}

impl View for AnalogStickView {
    fn setup(&mut self) {
        self.frame_mut().size = (SIZE, SIZE).into();

        self.enable_touch();

        let background = DrawingView::boxed();
        self.background = background.to_rglica();

        self.background.frame_mut().size = (SIZE, SIZE).into();

        let frame = *self.frame();
        self.background.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width),
            Color::BLACK,
        );

        self.background.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width - OUTLINE_WIDTH),
            Color::WHITE,
        );

        self.add_subview(background);

        let mut direction_stick = DrawingView::boxed();

        direction_stick.set_frame((STICK_VIEW_SIZE, STICK_VIEW_SIZE).into());

        direction_stick
            .frame_mut()
            .set_center(self.frame().size.center());

        let stick_center = direction_stick.frame().size.center();

        direction_stick.add_path(
            PointsPath::circle_with(stick_center, STICK_VIEW_SIZE),
            Color::BLACK,
        );

        direction_stick.add_path(
            PointsPath::circle_with(stick_center, STICK_VIEW_SIZE - OUTLINE_WIDTH),
            Color::LIGHT_GRAY,
        );

        self.direction_stick = direction_stick.to_rglica();

        self.add_subview(direction_stick);
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            if self.flaccid {
                return;
            }
            let frame = *self.frame();
            self.direction_stick
                .frame_mut()
                .set_center(frame.size.center());
            self.on_change.trigger(Point::default());
        } else {
            self.on_touch_moved(&touch.position);
        }
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
