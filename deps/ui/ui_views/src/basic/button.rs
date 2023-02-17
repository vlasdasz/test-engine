use gm::Color;
use refs::Weak;
use ui::{view, Event, SubView, ViewSetup, ViewSubviews, ViewTouch};

use crate::Label;

#[view]
#[derive(Default)]
pub struct Button {
    label_view: Option<SubView<Label>>,

    pub on_tap: Event,
}

impl Button {
    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        self.get_label().place.as_background();
        self.get_label().set_text(text);
        self
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.get_label().set_text_color(color);
        self
    }

    fn get_label(&mut self) -> &mut Label {
        if self.label_view.is_none() {
            let mut view: SubView<Label> = self.add_view();
            view.label = "Button.label_view".into();
            self.label_view = Some(view);
        }
        self.label_view.as_mut().unwrap()
    }
}

impl ViewSetup for Button {
    fn setup(self: Weak<Self>) {
        self.enable_touch();
        self.on_touch_began.sub(move || self.on_tap.trigger(()));
    }
}

#[macro_export]
macro_rules! link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {
        $self.$($button).+.on_tap.sub(move |_| $self.$method());
    }
}

#[macro_export]
macro_rules! async_link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {
        $self.$($button).+.on_tap.sub(move || {
            tokio::spawn(async move {
                $self.$method().await;
            });
        });
    };
}

#[macro_export]
macro_rules! async_call {
    ($self:ident, $method:ident) => {
        tokio::spawn(async move {
            $self.$method().await;
        });
    };
}
