use anyhow::Result;
use gm::Color;
use log::debug;
use refs::MainLock;
use render::{UIRectPipepeline, rect_instance::RectInstance, rect_view::RectView};
use test_engine::{
    RenderPass,
    ui::{UI, ViewCallbacks, view},
    ui_test::check_colors,
};
use window::Window;

static PIPELINE: MainLock<UIRectPipepeline> = MainLock::new();

#[view]
struct RenderTestView {}

impl ViewCallbacks for RenderTestView {
    fn before_render(&self, pass: &mut RenderPass) {
        let pipeline = PIPELINE.get_mut();

        pipeline.add(RectInstance::new((100, 100, 100, 100).into(), Color::RED, 0.5));
        pipeline.add(RectInstance::new((150, 150, 100, 100).into(), Color::GREEN, 0.5));
        pipeline.add(RectInstance::new((200, 200, 100, 100).into(), Color::BLUE, 0.5));

        pipeline.draw(
            pass,
            RectView {
                resolution: Window::current().size,
            },
        )
    }
}

pub async fn test_render() -> Result<()> {
    debug!("Test render");

    UI::init_test_view::<RenderTestView>().await;

    check_colors(
        r#"
              66  104 -  25  51  76
              77  109 -  25  51  76
             112  137 - 255   0   0
             134  152 - 255   0   0
             157  165 - 255   0   0
             195  196 - 255   0   0
             220  219 -   0 255   0
             270  266 -   0   0 203
             304  298 -  25  51  76
             340  260 -  25  51  76
             306  211 -  25  51  76
             225  187 -   0 255   0
             210   49 -  25  51  76
             283  105 -  25  51  76
             240  305 -  25  51  76
              60  279 -  25  51  76
             134  172 - 255   0   0
             143  136 - 255   0   0
             171   89 -  25  51  76
             269   78 -  25  51  76
             203  212 -   0 255   0
             132  250 -  25  51  76
             130  339 -  25  51  76
        "#,
    )
    .await?;

    debug!("OK");

    Ok(())
}
