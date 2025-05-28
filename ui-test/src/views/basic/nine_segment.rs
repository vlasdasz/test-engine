use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{NineSegmentImageView, Setup, UI, ViewData, view},
    ui_test::record_ui_test,
};

#[view]
struct NineSegment {
    #[init]
    segment: NineSegmentImageView,
}

impl Setup for NineSegment {
    fn setup(self: Weak<Self>) {
        self.segment.place().all_sides(100);

        self.segment.set_image("button");
    }
}

pub async fn test_nine_segment() -> Result<()> {
    let _view = UI::init_test_view::<NineSegment>().await;

    record_ui_test().await;

    Ok(())
}
