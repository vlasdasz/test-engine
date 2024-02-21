use anyhow::Result;
use test_engine::{ui::view, App};
use test_engine::refs::Weak;
use test_engine::ui::{Slider, SubView, ViewSetup};

#[view]
pub struct SliderTestView {
    slider: SubView<Slider>,
}

impl ViewSetup for SliderTestView {
    fn setup(self: Weak<Self>) {
        todo!()
    }
}

pub async fn test_slider() -> Result<()> {
    App::set_test_view::<SliderTestView>(600, 600).await;

    Ok(())
}
