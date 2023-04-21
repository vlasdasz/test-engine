use rtools::Apply;
use test_engine::gm::flat::Size;
use ui::{refs::Weak, view, Container, SubView, ViewSetup, ViewSubviews, ViewTest};

#[view]
struct BetweenTestView {
    center: SubView<Container>,

    top:    SubView<Container>,
    bottom: SubView<Container>,
    left:   SubView<Container>,
    right:  SubView<Container>,

    top_center:    SubView<Container>,
    bottom_center: SubView<Container>,
    left_center:   SubView<Container>,
    right_center:  SubView<Container>,
}

impl ViewSetup for BetweenTestView {
    fn setup(mut self: Weak<Self>) {
        for view in self.subviews_mut() {
            view.place.size(50, 50);
        }

        [self.center, self.top, self.bottom, self.left, self.right].apply(|view| {
            view.place.size(100, 100);
        });

        self.center.place.center();

        self.top.place.center_hor();
        self.bottom.place.b(0).center_hor();
        self.left.place.center_ver();
        self.right.place.center_ver().r(0);

        self.top_center.place.between(self.top, self.center);
        self.bottom_center.place.between(self.bottom, self.center);
        self.left_center.place.between(self.left, self.center);
        self.right_center.place.between(self.right, self.center);
    }
}

impl ViewTest for BetweenTestView {
    fn test_size() -> Size
    where Self: Sized {
        (1000, 1000).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<BetweenTestView>::start()
}
