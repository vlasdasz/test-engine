use refs::Weak;
use ui::{layout::Anchor, View};

pub trait WithHeader: View {
    fn header(&self) -> Weak<dyn View>;
    fn main_view(&self) -> Weak<dyn View>;
    fn header_size(&self) -> f32 {
        50.0
    }
    fn header_margin(&self) -> f32 {
        0.0
    }
    fn layout_header(&self) {
        self.header().place.lrt(0).h(self.header_size());
        self.main_view()
            .place
            .anchor(self.header(), Anchor::Top, self.header_margin())
            .lrb(0);
    }
}
