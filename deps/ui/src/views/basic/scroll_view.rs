use std::ops::Neg;

use gm::{
    ToF32,
    flat::{Point, Size},
};
use refs::{Weak, weak_from_ref};
use ui_proc::view;
use vents::Event;

use crate::{
    DELETED_VIEWS, NO_TOUCH_ID, Setup, Slider, Touch, TouchStack, UIAnimation, UIEvent, UIManager, View,
    ViewCallbacks,
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
    slider: Slider,
}

impl ScrollView {
    pub fn remove_all_subviews(&mut self) {
        let slider_addr = self.slider.raw();

        for mut view in self.subviews_mut() {
            if view.raw() == slider_addr {
                continue;
            }

            view.remove_from_superview();
        }
    }

    // Content offset must be negative
    fn max_offset(&self) -> f32 {
        (self.content_size.height - self.height()).neg().min(0.0)
    }

    pub fn set_content_offset(&mut self, offset: impl ToF32) -> &mut Self {
        self.__view_base.content_offset = offset.to_f32();

        if self.__view_base.content_offset < self.max_offset() {
            self.__view_base.content_offset = self.max_offset();
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

        if self.__view_base.content_offset < self.max_offset() {
            self.__view_base.content_offset = self.max_offset();
        }

        self
    }
}

impl Setup for ScrollView {
    fn setup(mut self: Weak<Self>) {
        self.__view_base.dont_hide_off_screen = true;
        self.slider.place().w(40).r(0);
        self.slider.on_change.val(move |val| {
            let val = 1.0 - val;
            let range = self.content_size.height - self.height();
            self.__view_base.content_offset = -range * val;
            self.on_scroll.trigger(self.__view_base.content_offset);
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
        let to_remove = self
            .__base_view()
            .subviews
            .extract_if(.., move |v| v.raw() != self.slider.raw());
        DELETED_VIEWS.lock().extend(to_remove);
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

        let mut target_frame = self.__view_base.absolute_frame;
        target_frame.origin.y -= self.__view_base.content_offset;

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

        dbg!(&self.inertia);

        let mut scroll = weak_from_ref(self);

        let anim = UIAnimation::new(move |_, _| {
            let inertia = scroll.inertia;
            scroll.on_scroll(inertia);
            scroll.inertia *= 0.9;
            dbg!(&scroll.inertia);
        })
        .finish_condition(move || scroll.inertia == 0.0);

        anim.on_finish.sub(|| {
            dbg!("No anime((");
        });

        self.add_animation(anim);
        dbg!("ANIMEE");
    }

    fn on_scroll(&mut self, scroll: f32) {
        if self.height() >= self.content_size.height {
            return;
        }
        self.__view_base.content_offset += scroll;
        let range = self.content_size.height - self.height();

        if self.__view_base.content_offset <= -range {
            self.bottom_reached.trigger(());
        }

        self.__view_base.content_offset = self.__view_base.content_offset.clamp(-range, 0.0);
        let slider_val = -self.__view_base.content_offset / range;
        self.slider.set_value_without_event(1.0 - slider_val);

        self.on_scroll.trigger(self.__view_base.content_offset);
    }
}
