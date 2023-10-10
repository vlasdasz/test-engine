use gl_image::ToImage;
use gm::Color;
use refs::Weak;
use ui::{view, Event, SubView, ToLabel, ViewSetup, ViewTouch};

use crate::{ImageView, Label};

#[view]
pub struct Button {
    label: SubView<Label>,
    image: SubView<ImageView>,

    pub on_tap: Event,
}

impl Button {
    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.label.is_hidden = false;
        self.label.set_text(text);
        self
    }

    pub fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.image.is_hidden = false;
        self.image.image = image.to_image();
        self
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.label.set_text_color(color);
        self
    }
}

impl ViewSetup for Button {
    fn setup(mut self: Weak<Self>) {
        self.label.place.back();
        self.label.is_hidden = true;

        self.image.place.back();
        self.image.is_hidden = true;

        self.enable_touch();
        self.touch.up_inside.sub(move || self.on_tap.trigger(()));
    }
}

#[macro_export]
macro_rules! _ui_link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {{
        use $crate::complex::AlertErr;
        $self.$($button).+.on_tap.sub(move || $self.$method().alert_err());
    }}
}

#[macro_export]
macro_rules! link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {{
        use ui_views::AlertErr;
        $self.$($button).+.on_tap.sub(move || $self.$method().alert_err());
    }}
}

#[macro_export]
macro_rules! async_link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {
        $self.$($button).+.on_tap.sub(move || {
            tokio::spawn(async move {
                use ui_views::AlertErr;
                $self.$method().await.alert_err();
            });
        });
    };
}

#[macro_export]
macro_rules! async_call {
    ($self:ident, $method:ident) => {
        tokio::spawn(async move {
            $self.$method().await.alert_err();
        });
    };
}
