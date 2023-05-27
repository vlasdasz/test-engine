use refs::Weak;
use rtools::platform::Platform;

use crate::{layout::Anchor, View};

pub trait WithHeader: View {
    fn header(&self) -> Weak<dyn View>;
    fn main_view(&self) -> Weak<dyn View>;
    fn header_size(&self) -> f32;
    fn header_margin(&self) -> f32;
    fn layout_header(&self) {
        if self.header().is_null() {
            return;
        }
        if Platform::IOS {
            self.header().place.lr(0).t(40).h(self.header_size());
        } else {
            self.header().place.lrt(0).h(self.header_size());
        }
        self.main_view()
            .place
            .anchor(self.header(), Anchor::Top, self.header_margin())
            .lrb(0);
    }
}

impl<T: View> WithHeader for T {
    default fn header(&self) -> Weak<dyn View> {
        Default::default()
    }

    default fn main_view(&self) -> Weak<dyn View> {
        unreachable!()
    }

    default fn header_margin(&self) -> f32 {
        0.0
    }

    default fn header_size(&self) -> f32 {
        50.0
    }
}
