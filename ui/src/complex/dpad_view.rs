use crate::basic::Button;
use crate::{View, ViewBase};
use gl_image::Image;
use gm::Rect;
use std::any::Any;
use tools::has_new::new;
use tools::refs::{new_shared, Shared};
use tools::Event;
use tools::{AsAny, New};

#[derive(Debug)]
pub struct DPadView {
    base: ViewBase,
    up: Shared<Button>,
    down: Shared<Button>,
    left: Shared<Button>,
    right: Shared<Button>,
    pub on_up: Event,
    pub on_down: Event,
    pub on_left: Event,
    pub on_right: Event,
}

impl DPadView {
    pub fn set_images(&self, up: Image, down: Image, left: Image, right: Image) {
        self.up.borrow_mut().image = up.into();
        self.down.borrow_mut().image = down.into();
        self.left.borrow_mut().image = left.into();
        self.right.borrow_mut().image = right.into();
    }
}

impl View for DPadView {
    fn setup(&mut self, this: Shared<dyn View>) {
        self.add_subview(self.up.clone());
        self.add_subview(self.down.clone());
        self.add_subview(self.left.clone());
        self.add_subview(self.right.clone());

        let a = this.clone();
        self.up.borrow_mut().on_tap.subscribe(move |_| {
            let this = a.borrow();
            let this = this.as_any().downcast_ref::<Self>().unwrap();
            this.on_up.trigger(&());
        });

        let a = this.clone();
        self.down.borrow_mut().on_tap.subscribe(move |_| {
            let this = a.borrow();
            let this = this.as_any().downcast_ref::<Self>().unwrap();
            this.on_down.trigger(&());
        });

        let a = this.clone();
        self.left.borrow_mut().on_tap.subscribe(move |_| {
            let this = a.borrow();
            let this = this.as_any().downcast_ref::<Self>().unwrap();
            this.on_left.trigger(&());
        });

        let a = this.clone();
        self.right.borrow_mut().on_tap.subscribe(move |_| {
            let this = a.borrow();
            let this = this.as_any().downcast_ref::<Self>().unwrap();
            this.on_right.trigger(&());
        });
    }

    fn layout(&mut self, _super_frame: &Rect) {
        let frame = self.frame();
        let third = frame.width() / 3.0;
        let half = frame.height() / 2.0;

        self.up
            .borrow_mut()
            .set_frame((third, 0, third, half).into());
        self.down
            .borrow_mut()
            .set_frame((third, half, third, half).into());
        self.left
            .borrow_mut()
            .set_frame((0, half, third, half).into());
        self.right
            .borrow_mut()
            .set_frame((third * 2.0, half, third, half).into());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl New for DPadView {
    fn new() -> DPadView {
        DPadView {
            base: new(),
            up: new_shared(),
            down: new_shared(),
            left: new_shared(),
            right: new_shared(),
            on_up: new(),
            on_down: new(),
            on_left: new(),
            on_right: new(),
        }
    }
}

impl AsAny for DPadView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
