use std::ops::Neg;

use gm::{
    ToF32,
    flat::{Point, Size},
};
use refs::{Own, Weak, weak_from_ref};
use ui_proc::view;
use vents::Event;

use crate::{
    Container, NO_TOUCH_ID, Setup, Slider, Touch, TouchStack, UIAnimation, UIEvent, UIManager, View,
    ViewCallbacks, WeakView,
    view::{ViewData, ViewFrame, ViewSubviews},
};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct ScrollView {
    inertia:            f32,
    previous_touch:     Point,
    content_size:       Size,
    pub on_scroll:      Event<f32>,
    pub bottom_reached: UIEvent,

    #[init]
    slider:    Slider,
    container: Container,
}

impl ScrollView {
    pub fn remove_all_subviews(&mut self) {
        self.container.remove_all_subviews();
    }

    // Content offset must be negative
    fn max_offset(&self) -> f32 {
        (self.content_size.height - self.height()).neg().min(0.0)
    }

    pub fn set_content_offset(&mut self, offset: impl ToF32) -> &mut Self {
        self.container.__base_view().content_offset = offset.to_f32();

        if self.container.__base_view().content_offset < self.max_offset() {
            self.container.__base_view().content_offset = self.max_offset();
        }

        self
    }

    pub fn set_content_size(&mut self, size: impl Into<Size>) -> &mut Self {
        self.content_size = size.into();
        self
    }

    pub fn set_content_width(&mut self, width: impl ToF32) -> &mut Self {
        self.content_size.width = width.to_f32();
        self
    }

    pub fn set_content_height(&mut self, height: impl ToF32) -> &mut Self {
        self.content_size.height = height.to_f32();

        if self.container.__base_view().content_offset < self.max_offset() {
            self.container.__base_view().content_offset = self.max_offset();
        }

        self
    }
}

impl Setup for ScrollView {
    fn setup(mut self: Weak<Self>) {
        self.container.__base_view().dont_hide_off_screen = true;
        self.slider.place().w(40).r(0);
        self.slider.on_change.val(move |val| {
            let val = 1.0 - val;
            let range = self.content_size.height - self.height();
            self.container.__base_view().content_offset = -range * val;
            self.on_scroll.trigger(self.container.__base_view().content_offset);
        });

        UIManager::on_scroll(self, move |scroll| {
            self.on_scroll(scroll.y);
        });

        self.size_changed().sub(move || {
            self.on_scroll(0.0);
        });

        TouchStack::enable_scroll(self);
    }
}

impl ViewCallbacks for ScrollView {
    fn update(&mut self) {
        let co = self.__view_base.content_offset;
        self.slider.set_y(-co);
        let height = self.height();
        self.slider.set_height(height);
        let hidden = self.height() >= self.content_size.height;
        self.slider.set_hidden(hidden);
    }

    fn content_size(&self) -> &Size {
        &self.content_size
    }
}

impl ViewSubviews for ScrollView {
    fn remove_all_subviews(&self) {
        self.container.remove_all_subviews();
    }

    fn add_subview<V: ?Sized + View + 'static>(&self, view: Own<V>) -> Weak<V> {
        self.container.__add_subview_internal(view, false)
    }
}

impl ScrollView {
    pub fn __process_scroll_touch(&mut self, touch: Touch) -> bool {
        if touch.is_ended() {
            if touch.id == self.__view_base.touch_id {
                self.add_inertia_animation();
            }

            self.__view_base.touch_id = NO_TOUCH_ID;
            return false;
        }

        let mut target_frame = self.container.__base_view().absolute_frame;
        target_frame.origin.y -= self.container.__base_view().content_offset;

        if touch.is_began() && target_frame.contains(touch.position) {
            self.__view_base.touch_id = touch.id;
            self.previous_touch = touch.position;
            return true;
        }

        if touch.is_moved() && self.__view_base.touch_id == touch.id {
            let delta = -(self.previous_touch.y - touch.position.y);
            self.previous_touch = touch.position;

            if delta == 0.0 {
                return true;
            }

            self.inertia = delta;
            self.on_scroll(delta);
            return true;
        }

        false
    }

    fn add_inertia_animation(&self) {
        if self.inertia == 0.0 {
            return;
        }

        let mut scroll = weak_from_ref(self);

        let anim = UIAnimation::new(move |_, _| {
            let inertia = scroll.inertia;
            scroll.on_scroll(inertia);
            scroll.inertia *= 0.97;
        })
        .finish_condition(move || scroll.inertia.abs() <= 0.2);

        self.add_animation(anim);
    }

    fn on_scroll(&mut self, scroll: f32) {
        if self.height() >= self.content_size.height {
            return;
        }
        self.container.__base_view().content_offset += scroll;
        let range = self.content_size.height - self.height();

        if self.container.__base_view().content_offset <= -range {
            self.bottom_reached.trigger(());
        }

        self.container.__base_view().content_offset =
            self.container.__base_view().content_offset.clamp(-range, 0.0);
        let slider_val = -self.container.__base_view().content_offset / range;
        self.slider.set_value_without_event(1.0 - slider_val);

        self.on_scroll.trigger(self.container.__base_view().content_offset);
    }
}
