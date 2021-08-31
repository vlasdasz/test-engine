use crate::basic::Button;
use crate::{Label, Layout, View, ViewBase};
use gm::Rect;
use proc_macro::AsAny;
use std::cell::RefCell;
use std::ops::AddAssign;
use tools::has_new::new;
use tools::refs::{new_shared, Shared};
use tools::{Event, New};

#[derive(AsAny)]
pub struct IntView {
    base: ViewBase,
    value: RefCell<i64>,
    label: Shared<Label>,
    up: Shared<Button>,
    down: Shared<Button>,
    pub on_change: Event<i64>,
}

impl View for IntView {
    fn setup(&mut self, this: Shared<dyn View>) {
        self.add_subview(self.label.clone());
        self.add_subview(self.up.clone());
        self.add_subview(self.down.clone());

        // self.up.borrow_mut().image = Some(Image::load(&paths::images().join("up.png")));
        // self.down.borrow_mut().image = Some(Image::load(&paths::images().join("down.png")));

        let a = this.clone();
        self.up.borrow_mut().on_tap.subscribe(move |_| {
            let this = a.borrow();
            let this = this.as_any().downcast_ref::<Self>().unwrap();
            this.value.borrow_mut().add_assign(1);
            this.on_change.trigger(&this.value.borrow());
        });

        let a = this.clone();
        self.down.borrow_mut().on_tap.subscribe(move |_| {
            let this = a.borrow();
            let this = this.as_any().downcast_ref::<Self>().unwrap();
            this.value.borrow_mut().add_assign(-1);
            this.on_change.trigger(&this.value.borrow());
        });
    }

    fn update(&mut self) {
        self.label
            .borrow_mut()
            .set_text(&self.value.borrow().to_string());
    }

    fn layout(&mut self, _super_frame: &Rect) {
        Layout::distribute_vertically(&self.frame().clone(), self.subviews_mut());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl New for IntView {
    fn new() -> Self {
        IntView {
            base: new(),
            value: new(),
            label: new_shared(),
            up: new_shared(),
            down: new_shared(),
            on_change: new(),
        }
    }
}
