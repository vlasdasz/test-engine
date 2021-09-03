use crate::basic::{Button, Placer};
use crate::{Label, View, ViewBase, make_view_on};
use proc_macro::AsAny;
use proc_macro::Boxed;
use std::cell::RefCell;
use std::ops::AddAssign;
use tools::has_new::new;
use tools::refs::{new_shared, Shared};
use tools::{Event, New, Rglica};

#[derive(AsAny, Boxed)]
pub struct IntView {
    base: ViewBase,
    value: i64,
    label: Rglica<Label>,
    up: Rglica<Button>,
    down: Rglica<Button>,
    pub on_change: Event<i64>,
}

impl View for IntView {
    fn setup(&mut self) {

        self.label = make_view_on(self);
        self.up = make_view_on(self);
        self.down = make_view_on(self);

        let mut this = Rglica::from_ref(self);
        self.up.on_tap.subscribe(move |_| {
            this.value.add_assign(1);
            let val = this.value;
            this.on_change.trigger(&val);
        });

        let mut this = Rglica::from_ref(self);
        self.down.on_tap.subscribe(move |_| {
            this.value.add_assign(-1);
            let val = this.value;
            this.on_change.trigger(&val);
        });
    }

    fn update(&mut self) {
        self.label
            .set_text(&self.value.to_string());
    }

    fn layout(&mut self) {
        Placer::distribute_vertically(&self.frame().clone(), self.subviews_mut());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
