use std::{
    fmt::{Debug, Display},
    num::NonZeroU32,
};

use gm::{CheckedSub, MyAdd, One, ZeroOrMinimal};
use refs::{weak_from_ref, Weak};
use vents::Event;

use crate::{view::ViewData, Button, InputView, Label, UIImages, ViewSetup, ViewTouch};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use ui_proc::view;

use crate::Sub;

pub trait ViewableNumber:
    MyAdd + CheckedSub + ZeroOrMinimal + One + Copy + Debug + Display + Sized + 'static {
}

impl ViewableNumber for f32 {}
impl ViewableNumber for u32 {}
impl ViewableNumber for NonZeroU32 {}

#[view]
pub struct NumberView<T: ViewableNumber> {
    #[educe(Default = T::one())]
    value:    T,
    #[educe(Default = T::one())]
    pub step: T,
    #[educe(Default = ZeroOrMinimal::zero())]
    pub min:  T,

    label: Sub<Label>,

    #[link = up_tap]
    up: Sub<Button>,

    #[link = down_tap]
    down: Sub<Button>,

    on_change_event: Event<T>,
}

impl<T: ViewableNumber> ViewSetup for NumberView<T> {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();
        self.label.text = format!("{:.1}", self.value);
        self.up.set_image(UIImages::up());
        self.down.set_image(UIImages::down());
    }
}

impl<T: ViewableNumber> NumberView<T> {
    pub fn value(&self) -> T {
        self.value
    }

    pub fn set_value(&mut self, val: T) -> &mut Self {
        self.value = val;
        self.label.text = format!("{val:.1}");
        self.on_change_event.trigger(val);
        self
    }

    pub fn set_step(&mut self, step: T) -> &mut Self {
        self.step = step;
        self
    }

    fn up_tap(mut self: Weak<Self>) {
        let val = self.value.my_add(&self.step);
        self.set_value(val);
    }

    fn down_tap(mut self: Weak<Self>) {
        let val = self.value.sub_and_check(&self.step, &self.min);
        self.set_value(val.unwrap_or(T::zero()));
    }

    pub fn on_change(&self, action: impl FnMut(T) + 'static) -> &Self {
        self.on_change_event.val(action);
        self
    }
}

impl<T: ViewableNumber> InputView for NumberView<T> {
    fn set_title(&mut self, _title: &str) {
        unimplemented!()
    }

    fn text(&self) -> &str {
        self.label.text()
    }

    fn enable_editing(&mut self) {
        self.up.enable_touch();
        self.down.enable_touch();
    }

    fn disable_editing(&mut self) {
        self.up.disable_touch();
        self.down.disable_touch();
    }

    fn as_input_view(&self) -> Weak<dyn InputView> {
        weak_from_ref(self as _)
    }
}
