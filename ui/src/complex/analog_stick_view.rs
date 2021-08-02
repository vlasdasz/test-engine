use crate::complex::DrawingView;
use crate::{View, ViewBase};
use gm::flat::PointsPath;
use gm::{Color, Point};
use proc_macro::AsAny;
use tools::refs::{new_shared, Shared};
use tools::{new, Event, New};

const SIZE: f32 = 140.0;
const OUTLINE_WIDTH: f32 = 10.0;
const STICK_VIEW_SIZE: f32 = SIZE / 2.0;

#[derive(Debug, AsAny)]
pub struct AnalogStickView {
    base: ViewBase,
    direction_stick: Shared<DrawingView>,
    background: Shared<DrawingView>,
    pub on_direction_change: Event<Point>,
}

impl AnalogStickView {
    fn on_touch_moved(&self, touch: &Point) {
        let max_lenght = self.frame().size.height / 2.0;
        let center = self.frame().size.center();

        let vector = (touch - &center).trimmed(max_lenght);

        self.direction_stick
            .borrow_mut()
            .frame_mut()
            .set_center(&(vector + self.frame().size.center()));

        self.on_direction_change.trigger(&(vector * 0.1));
    }
}

impl View for AnalogStickView {
    fn setup(&mut self, this: Shared<dyn View>) {
        self.frame_mut().size = (SIZE, SIZE).into();

        self.enable_touch();

        self.background.borrow_mut().frame_mut().size = (SIZE, SIZE).into();

        self.background.borrow_mut().add_path(
            PointsPath::circle_with(self.frame().size.center(), self.frame().size.width),
            Color::BLACK,
        );

        self.background.borrow_mut().add_path(
            PointsPath::circle_with(
                self.frame().size.center(),
                self.frame().size.width - OUTLINE_WIDTH,
            ),
            Color::WHITE,
        );

        self.add_subview(self.background.clone());

        let mut direction_stick = self.direction_stick.borrow_mut();

        direction_stick.set_frame((STICK_VIEW_SIZE, STICK_VIEW_SIZE).into());

        direction_stick
            .frame_mut()
            .set_center(&self.frame().size.center());

        let stick_center = direction_stick.frame().size.center();

        direction_stick.add_path(
            PointsPath::circle_with(stick_center, STICK_VIEW_SIZE),
            Color::BLACK,
        );

        direction_stick.add_path(
            PointsPath::circle_with(stick_center, STICK_VIEW_SIZE - OUTLINE_WIDTH),
            Color::LIGHT_GRAY,
        );

        drop(direction_stick);

        self.add_subview(self.direction_stick.clone());

        let a = this.clone();
        self.on_touch().subscribe(move |touch| {
            let this = a.borrow();
            let this = this.as_any().downcast_ref::<Self>().unwrap();
            if touch.is_ended() {
                this.direction_stick
                    .borrow_mut()
                    .frame_mut()
                    .set_center(&this.frame().size.center());
                this.on_direction_change.trigger(&Point::DEFAULT);
            } else {
                this.on_touch_moved(&touch.position);
            }
        });
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl New for AnalogStickView {
    fn new() -> Self {
        Self {
            base: new(),
            direction_stick: new_shared(),
            background: new_shared(),
            on_direction_change: new(),
        }
    }
}
