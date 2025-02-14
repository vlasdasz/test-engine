use anyhow::Result;
use gm::{Color, flat::Size};
use log::debug;
use refs::MainLock;
use render::{
    UIImageRectPipepeline, UIRectPipepeline, rect_view::RectView, ui_rect_instance::UIRectInstance,
};
use test_engine::{
    App, RenderPass, from_main,
    ui::{UI, UIImages, UIManager, ViewCallbacks, view},
    ui_test::check_colors,
};
use window::Window;

static UI_RECT: MainLock<UIRectPipepeline> = MainLock::new();
static IMAGE_DRAWER: MainLock<UIImageRectPipepeline> = MainLock::new();

#[view]
struct RenderTestView {
    case: u8,
}

impl RenderTestView {
    fn case_0(&self, pass: &mut RenderPass) {
        let rect = UI_RECT.get_mut();
        let image = IMAGE_DRAWER.get_mut();

        rect.add(UIRectInstance::new(
            (100, 100, 100, 100).into(),
            Color::RED,
            0.0,
            0.5,
        ));
        rect.add(UIRectInstance::new(
            (150, 150, 100, 100).into(),
            Color::GREEN,
            0.0,
            0.5,
        ));
        rect.add(UIRectInstance::new(
            (200, 200, 100, 100).into(),
            Color::BLUE,
            0.0,
            0.5,
        ));

        rect.add(UIRectInstance::new(
            (200, 500, 100, 100).into(),
            Color::BLUE,
            0.0,
            0.5,
        ));
        rect.add(UIRectInstance::new(
            (150, 450, 100, 100).into(),
            Color::GREEN,
            0.0,
            0.5,
        ));
        rect.add(UIRectInstance::new(
            (100, 400, 100, 100).into(),
            Color::RED,
            0.0,
            0.5,
        ));

        rect.add(UIRectInstance::new(
            (100, 700, 100, 100).into(),
            Color::RED,
            0.0,
            0.3,
        ));
        rect.add(UIRectInstance::new(
            (150, 750, 100, 100).into(),
            Color::GREEN,
            0.0,
            0.2,
        ));
        rect.add(UIRectInstance::new(
            (200, 800, 100, 100).into(),
            Color::BLUE,
            0.0,
            0.1,
        ));

        rect.add(UIRectInstance::new(
            (400, 100, 200, 200).into(),
            Color::GREEN,
            0.0,
            0.5,
        ));

        let size = Window::inner_size();
        let size: Size = (size.width, size.height).into();

        rect.draw(pass, RectView { resolution: size });

        image.add_with_image(
            UIRectInstance {
                position:      (450, 150).into(),
                size:          (100, 100).into(),
                color:         Default::default(),
                corner_radius: 0.0,
                z_position:    0.4,
            },
            UIImages::rb(),
        );

        image.draw(pass, RectView { resolution: size });

        let window_size = UIManager::resolution();

        pass.set_viewport(0.0, 0.0, window_size.width, window_size.height, 0.0, 1.0);
    }

    fn case_1(&self, _pass: &mut RenderPass) {
        let pipeline = UI_RECT.get_mut();

        pipeline.add(UIRectInstance::new(
            (200, 200, 100, 100).into(),
            Color::BLUE,
            0.0,
            0.5,
        ));
        pipeline.add(UIRectInstance::new(
            (150, 150, 100, 100).into(),
            Color::GREEN,
            0.0,
            0.5,
        ));
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
              55  693 -  25  51  76
              79  704 -  25  51  76
              94  726 -  25  51  76
             106  734 - 255   0   0
             120  746 - 255   0   0
             128  753 - 255   0   0
             151  763 -   0 255   0
             170  773 -   0 255   0
             170  775 -   0 255   0
             188  801 -   0 255   0
             218  826 -   0   0 203
             238  843 -   0   0 203
             283  875 -   0   0 203
             306  888 -  25  51  76
             340  906 -  25  51  76
             363  921 -  25  51  76
             324  583 -  25  51  76
             306  572 -  25  51  76
             273  561 -   0   0 203
             259  554 -   0   0 203
             254  549 -   0   0 203
             235  532 -   0   0 203
             232  529 -   0   0 203
             223  521 -   0   0 203
             214  514 -   0   0 203
             195  496 -   0 255   0
             190  489 -   0 255   0
             173  472 -   0 255   0
             160  460 -   0 255   0
             123  423 - 255   0   0
             124  423 - 255   0   0
              95  392 -  25  51  76
              93  389 -  25  51  76
              74   88 -  25  51  76
              77   93 -  25  51  76
              89  107 -  25  51  76
             111  121 - 255   0   0
             140  137 - 255   0   0
             143  140 - 255   0   0
             168  162 - 255   0   0
             187  181 - 255   0   0
             209  203 -   0 255   0
             223  226 -   0 255   0
             241  248 -   0 255   0
             243  249 -   0 255   0
             263  262 -   0   0 203
             298  297 -   0   0 203
             302  302 -  25  51  76
             348  229 -  25  51  76
             373  221 -  25  51  76
             443  213 -   0 255   0
             467  212 -   0 255   0
             497  216 -  14  14  14
             557  217 -   0 255   0
             629  219 -  25  51  76
             672  210 -  25  51  76
             666  150 -  25  51  76
             615  161 -  25  51  76
             538  165 -  14  14  14
             497  177 -   0 255   0
             375  160 -  25  51  76
             379  133 -  25  51  76
             393   83 -  25  51  76
             473   66 -  25  51  76
             488   92 -  25  51  76
             507  182 -   0 255   0
             523  278 -   0 255   0
             519  391 -  25  51  76
             179   66 -  25  51  76
             188  109 - 255   0   0
             219  244 -   0 255   0
             221  329 -  25  51  76
             205  398 -  25  51  76
             206  451 -   0 255   0
             192  582 -  25  51  76
             193  632 -  25  51  76
             234  748 -  25  51  76
             238  852 -   0   0 203
             226  913 -  25  51  76
             223  948 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.case = 0;
    })
    .await;

    debug!("OK");

    Ok(())
}
