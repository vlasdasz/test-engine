use crate::gm::Rect;
use crate::image::Image;
use crate::te::paths;
use crate::ui::basic::Button;
use crate::ui::{Label, Layout, View, ViewBase};
use std::any::Any;
use tools::has_new::new;
use tools::refs::{new_shared, Shared};
use tools::{AsAny, Event, New};

#[derive(Debug)]
pub struct IntView {
    base: ViewBase,
    value: i64,
    label: Shared<Label>,
    up: Shared<Button>,
    down: Shared<Button>,
    pub on_change: Event<i64>,
}

impl View for IntView {
    fn setup(&mut self, _: Shared<dyn View>) {
        self.add_subview(self.label.clone());
        self.add_subview(self.up.clone());
        self.add_subview(self.down.clone());

        self.up.borrow_mut().image = Some(Image::load(&paths::images().join("up.png")));
        self.down.borrow_mut().image = Some(Image::load(&paths::images().join("down.png")));

        //let a = this.clone();
        self.up.borrow_mut().on_tap.subscribe(move |_| {
            // let this = a.borrow();
            // let this = this.as_any().downcast_ref::<Self>().unwrap();
            // this.on_change.trigger(&50);
            //let mut this = a.borrow_mut();
            // let mut this = this.as_any_mut().downcast_mut::<Self>().unwrap();
            // this.value += 1;
            // this.on_change.trigger(&this.value);
        });

        // let a = this.clone();
        // self.down.borrow_mut().on_tap.subscribe(move |_| {
        //     let mut this = a.borrow_mut();
        //     let mut this = this.as_any_mut().downcast_mut::<Self>().unwrap();
        //     this.value -= 1;
        //     this.on_change.trigger(&this.value);
        // });
    }

    fn update(&mut self) {
        self.label.borrow_mut().set_text(&self.value.to_string());
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
            value: 0,
            label: new_shared(),
            up: new_shared(),
            down: new_shared(),
            on_change: new(),
        }
    }
}

impl AsAny for IntView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
