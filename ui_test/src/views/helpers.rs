use test_engine::{
    ui::{Color, Container, ViewData, ViewSubviews, WeakView},
    Apply,
};

pub fn add_corners(mut view: WeakView, color: Color) {
    let _1 = view.add_view::<Container>();
    let _2 = view.add_view::<Container>();
    let _3 = view.add_view::<Container>();
    let _4 = view.add_view::<Container>();

    [_1, _2, _3, _4].apply(|mut a| {
        a.place().size(100, 100);
        a.set_color(color);
    });

    _1.place().tl(0);
    _2.place().tr(0);
    _3.place().bl(0);
    _4.place().br(0);
}
