use gm::Color;
use refs::Weak;
use rtools::platform::Platform;

use crate::{layout::Anchor, View, ViewData};

pub trait WithHeader: View {
    fn header(&self) -> Weak<dyn View>;
    fn main_view(&self) -> Weak<dyn View>;
    fn header_size(&self) -> f32;
    fn header_margin(&self) -> f32;
    fn layout_header(&self) {
        let mut header = self.header();

        if header.is_null() {
            return;
        }

        header.set_color(Color::WHITE);

        if Platform::IOS {
            header.place().lr(0).t(40).h(self.header_size());
        } else {
            header.place().lrt(0).h(self.header_size());
        }

        if !self.main_view().is_null() {
            self.main_view()
                .place()
                .anchor(Anchor::Top, header, self.header_margin())
                .lrb(0);
        }
    }
}

impl<T: View> WithHeader for T {
    default fn header(&self) -> Weak<dyn View> {
        Default::default()
    }

    default fn main_view(&self) -> Weak<dyn View> {
        Default::default()
    }

    default fn header_size(&self) -> f32 {
        50.0
    }

    default fn header_margin(&self) -> f32 {
        0.0
    }
}
