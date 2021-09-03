use crate::{basic::Button, View, ViewBase};
use gl_image::Image;
use proc_macro::AsAny;
use proc_macro::Boxed;
use tools::rglica::ToRglica;
use tools::Boxed;
use tools::{
    has_new::new,
    refs::{new_shared, Shared},
    Event, New, Rglica,
};
use glfw::MouseButton::Button1;

#[derive(AsAny, Boxed)]
pub struct DPadView {
    base: ViewBase,
    up: Rglica<Button>,
    down: Rglica<Button>,
    left: Rglica<Button>,
    right: Rglica<Button>,
    pub on_up: Event,
    pub on_down: Event,
    pub on_left: Event,
    pub on_right: Event,
}

impl DPadView {
    pub fn set_images(&mut self, up: Image, down: Image, left: Image, right: Image) {
        self.up.image = up.into();
        self.down.image = down.into();
        self.left.image = left.into();
        self.right.image = right.into();
    }
}

impl View for DPadView {
    fn setup(&mut self) {
        let mut up = Button::boxed();
        let mut down = Button::boxed();
        let mut left = Button::boxed();
        let mut right = Button::boxed();

        self.up = up.to_rglica();
        self.down = down.to_rglica();
        self.left = left.to_rglica();
        self.right = right.to_rglica();

        self.add_subview(up);
        self.add_subview(down);
        self.add_subview(left);
        self.add_subview(right);

        let mut a = Rglica::from_ref(self);
        self.up.on_tap.subscribe(move |_| {
            a.on_up.trigger(&());
        });

        let mut a = Rglica::from_ref(self);
        self.down.on_tap.subscribe(move |_| {
            a.on_down.trigger(&());
        });

        let mut a = Rglica::from_ref(self);
        self.left.on_tap.subscribe(move |_| {
            a.on_left.trigger(&());
        });

        let mut a = Rglica::from_ref(self);
        self.right.on_tap.subscribe(move |_| {
            a.on_right.trigger(&());
        });
    }

    fn layout(&mut self) {
        let frame = self.frame();
        let third = frame.width() / 3.0;
        let half = frame.height() / 2.0;

        self.up.set_frame((third, 0, third, half).into());
        self.down.set_frame((third, half, third, half).into());
        self.left.set_frame((0, half, third, half).into());
        self.right
            .set_frame((third * 2.0, half, third, half).into());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
