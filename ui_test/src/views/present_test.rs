use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Label, ModalView, NavigationView, Size, SubView, ViewData, ViewSetup, ViewSubviews},
    wait_for_next_frame, App, OnceEvent,
};

use crate::views::image_view::check_colors;

#[view]
struct PresentTestView {}

impl ViewSetup for PresentTestView {
    fn setup(mut self: Weak<Self>) {}
}

pub async fn test_present() -> Result<()> {
    App::set_test_view(NavigationView::with_view(PresentTestView::new()), 600, 600).await;

    debug!("Present test: OK");

    Ok(())
}
