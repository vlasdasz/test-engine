use old_engine::gm::Color;
use rtools::Apply;

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

    top_s_center:    SubView<Container>,
    bottom_s_center: SubView<Container>,
    left_s_center:   SubView<Container>,
    right_s_center:  SubView<Container>,
}

impl ViewSetup for BetweenTestView {
    fn setup(mut self: Weak<Self>) {
        for view in self.subviews_mut() {
            view.place().size(50, 50);
        }

        [self.center, self.top, self.bottom, self.left, self.right].apply(|view| {
            view.place().size(100, 100);
        });

        self.center.place().center();

        self.top.set_color(Color::ORANGE).place().center_x().t(200);
        self.bottom.set_color(Color::GREEN).place().center_x().b(200);
        self.left.place().center_y().l(200);
        self.right.place().center_y().r(200);

        self.top_center.place().between(self.top, self.center);
        self.bottom_center.place().between(self.bottom, self.center);
        self.left_center.place().between(self.left, self.center);
        self.right_center.place().between(self.right, self.center);

        self.top_s_center.place().between_super(self.top, Anchor::Top);
        self.bottom_s_center.place().between_super(self.bottom, Anchor::Bot);
        self.left_s_center.place().between_super(self.left, Anchor::Left);
        self.right_s_center.place().between_super(self.right, Anchor::Right);
    }
}

#[ignore]
#[test]
fn test() {
    old_engine::ViewApp::<BetweenTestView>::start().unwrap()
}
