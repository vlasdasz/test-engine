use anyhow::Result;
use gm::Color;
use log::debug;
use refs::MainLock;
use render::{UIRectPipepeline, rect_instance::RectInstance, rect_view::RectView};
use test_engine::{
    App, DataManager, RenderPass, from_main,
    ui::{UI, UIImages, UIManager, ViewCallbacks, view},
    ui_test::{check_colors, record_ui_test},
};
use window::{ImageDrawer, Window};

static UI_RECT: MainLock<UIRectPipepeline> = MainLock::new();
static IMAGE_DRAWER: MainLock<ImageDrawer> = MainLock::new();

#[view]
struct RenderTestView {
    case: u8,
}

impl RenderTestView {
    fn case_0(&self, pass: &mut RenderPass) {
        let rect = UI_RECT.get_mut();
        let image = IMAGE_DRAWER.get_mut();

        rect.add(RectInstance::new((100, 100, 100, 100).into(), Color::RED, 0.5));
        rect.add(RectInstance::new((150, 150, 100, 100).into(), Color::GREEN, 0.5));
        rect.add(RectInstance::new((200, 200, 100, 100).into(), Color::BLUE, 0.5));

        rect.add(RectInstance::new((200, 500, 100, 100).into(), Color::BLUE, 0.5));
        rect.add(RectInstance::new((150, 450, 100, 100).into(), Color::GREEN, 0.5));
        rect.add(RectInstance::new((100, 400, 100, 100).into(), Color::RED, 0.5));

        rect.add(RectInstance::new((100, 700, 100, 100).into(), Color::RED, 0.3));
        rect.add(RectInstance::new((150, 750, 100, 100).into(), Color::GREEN, 0.2));
        rect.add(RectInstance::new((200, 800, 100, 100).into(), Color::BLUE, 0.1));

        rect.add(RectInstance::new((400, 100, 200, 200).into(), Color::GREEN, 0.5));

        rect.draw(
            pass,
            RectView {
                resolution: Window::current().size,
            },
        );

        image.draw(
            pass,
            UIImages::rb().get_static(),
            &(450, 150, 100, 100).into(),
            None,
            0.4,
        );

        let window_size = UIManager::resolution();

        pass.set_viewport(0.0, 0.0, window_size.width, window_size.height, 0.0, 1.0);
    }

    fn case_1(&self, pass: &mut RenderPass) {
        let pipeline = UI_RECT.get_mut();

        pipeline.add(RectInstance::new((200, 200, 100, 100).into(), Color::BLUE, 0.5));
        pipeline.add(RectInstance::new((150, 150, 100, 100).into(), Color::GREEN, 0.5));
        pipeline.add(RectInstance::new((100, 100, 100, 100).into(), Color::RED, 0.5));

        pipeline.draw(
            pass,
            RectView {
                resolution: Window::current().size,
            },
        )
    }
}

impl ViewCallbacks for RenderTestView {
    fn before_render(&self, pass: &mut RenderPass) {
        match self.case {
            0 => self.case_0(pass),
            1 => self.case_1(pass),
            _ => panic!(),
        }
    }
}

pub async fn test_render() -> Result<()> {
    debug!("Test render");

    let mut view = UI::init_test_view::<RenderTestView>().await;

    App::set_window_size((1000, 1000)).await;

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

    from_main(move || {
        view.case = 0;
    })
    .await;

    record_ui_test().await;

    debug!("OK");

    Ok(())
}
