use crate::gm::flat::PointsPath;
use crate::gm::{Color, Rect};
use crate::ui::complex::DrawingView;
use crate::ui::{View, ViewBase, Event};
use std::any::Any;
use tools::refs::{new_shared, Shared};
use tools::{new, AsAny, New};
use crate::ui::input::Touch;
use crate::image::Image;

const OUTLINE_WIDTH: f32 = 10.0;

#[derive(Debug)]
pub struct AnalogStickView {
    base: ViewBase,
    direction_stick: Shared<DrawingView>,
    background: Shared<DrawingView>,
}

impl View for AnalogStickView {
    fn setup(&mut self, _this: Shared<dyn View>) {
        self.enable_touch();

        self.background.borrow_mut().add_path(
            PointsPath::circle_with(self.frame().size.center(), self.frame().size.width, 50),
            Color::BLACK,
        );

        self.background.borrow_mut().add_path(
            PointsPath::circle_with(
                self.frame().size.center(),
                self.frame().size.width - OUTLINE_WIDTH,
                50,
            ),
            Color::WHITE,
        );

        let STICK_VIEW_SIZE = self.frame().size.width / 2.0;

        self.direction_stick
            .borrow_mut()
            .set_frame((STICK_VIEW_SIZE, STICK_VIEW_SIZE).into());

        self.add_subview(self.direction_stick.clone());
        
        //
        // auto stick_center = direction_stick->frame().size.center();
        //
        // direction_stick->add_path(
        //     PointsPath::circle_with(stick_center, STICK_VIEW_SIZE),
        //     Color::black);
        //
        // direction_stick->add_path(
        //     PointsPath::circle_with(stick_center, STICK_VIEW_SIZE - OUTLINE_WIDTH),
        //     Color::light_gray);
    }

    fn layout(&mut self, _super_frame: &Rect) {
        // direction_stick->place.set_center(_frame.size.center());

    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl AsAny for AnalogStickView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl New for AnalogStickView {
    fn new() -> Self {
        Self {
            base: new(),
            direction_stick: new_shared(),
            background: new_shared(),
        }
    }
}
