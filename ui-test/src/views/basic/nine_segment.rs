use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Color, Image, ImageView, Setup, UI, ViewCallbacks, ViewData, ViewFrame, ViewSubviews, view},
    ui_test::record_ui_test,
};

#[view]
struct NineSegmentImageView {
    _corner: Weak<Image>,
    _side:   Weak<Image>,
    _center: Weak<Image>,

    #[educe(Default = 100.0)]
    pub corner_size: f32,

    #[init]
    tl: ImageView,
    tr: ImageView,
    bl: ImageView,
    br: ImageView,

    t: ImageView,
    b: ImageView,
    r: ImageView,
    l: ImageView,

    c: ImageView,
}

impl Setup for NineSegmentImageView {
    fn setup(self: Weak<Self>) {}
}

impl ViewCallbacks for NineSegmentImageView {
    fn update(&mut self) {
        let w = self.width();
        let h = self.height();

        let mut cs = self.corner_size;

        let smaller_side = if w < h { w } else { h };

        if cs > smaller_side / 2.0 {
            cs = smaller_side / 2.0;
        };

        self.tl.set_frame((0, 0, cs, cs));
        self.tr.set_frame((w - cs, 0, cs, cs));
        self.bl.set_frame((0, h - cs, cs, cs));
        self.br.set_frame((w - cs, h - cs, cs, cs));

        let middle_w = w - cs * 2.0;
        let middle_h = h - cs * 2.0;

        self.t.set_frame((cs, 0, middle_w, cs));
        self.b.set_frame((cs, h - cs, middle_w, cs));
        self.r.set_frame((w - cs, cs, cs, middle_h));
        self.l.set_frame((0, cs, cs, middle_h));

        self.c.set_frame((cs, cs, middle_w, middle_h));
    }
}

#[view]
struct NineSegment {
    #[init]
    segment: NineSegmentImageView,
}

impl Setup for NineSegment {
    fn setup(mut self: Weak<Self>) {
        self.segment.place().back();

        self.segment.subviews_mut().iter_mut().for_each(|v| {
            v.set_color(Color::random());
        });
    }
}

pub async fn test_nine_segment() -> Result<()> {
    let _view = UI::init_test_view::<NineSegment>().await;

    record_ui_test().await;

    Ok(())
}
