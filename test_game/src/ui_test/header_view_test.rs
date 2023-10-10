use test_engine::gm::Color;
use ui::{refs::Weak, view, Container, SubView, View, ViewData, ViewSetup, WithHeader};

#[view]
struct HeaderTestView {
    header: SubView<Container>,
    main:   SubView<Container>,
}

impl ViewSetup for HeaderTestView {
    fn setup(mut self: Weak<Self>) {
        self.header.set_color(Color::GREEN);
        self.main.set_color(Color::BLUE);
    }
}

impl WithHeader for HeaderTestView {
    fn header(&self) -> Weak<dyn View> {
        self.header.weak_view()
    }

    fn main_view(&self) -> Weak<dyn View> {
        self.main.weak_view()
    }

    fn header_size(&self) -> f32 {
        100.0
    }

    fn header_margin(&self) -> f32 {
        10.0
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<HeaderTestView>::start()
}
