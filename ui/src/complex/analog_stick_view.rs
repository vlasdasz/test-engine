use gm::{flat::PointsPath, Color, Point};
use proc_macro::{AsAny, Boxed};
use tools::{rglica::ToRglica, Boxed, Event, Rglica};

use crate::{complex::DrawingView, View, ViewBase};

const SIZE: f32 = 140.0;
const OUTLINE_WIDTH: f32 = 10.0;
const STICK_VIEW_SIZE: f32 = SIZE / 2.0;

#[derive(AsAny, Boxed)]
pub struct AnalogStickView {
    base:                    ViewBase,
    direction_stick:         Rglica<DrawingView>,
    background:              Rglica<DrawingView>,
    pub on_direction_change: Event<Point>,
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

        self.on_direction_change.trigger(vector * 0.1);
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

        let mut this = Rglica::from_ref(self);
        self.on_touch().subscribe(move |touch| {
            if touch.is_ended() {
                let frame = *this.frame();
                this.direction_stick
                    .frame_mut()
                    .set_center(frame.size.center());
                this.on_direction_change.trigger(Point::DEFAULT);
            } else {
                this.on_touch_moved(&touch.position);
            }
        });
    }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}
