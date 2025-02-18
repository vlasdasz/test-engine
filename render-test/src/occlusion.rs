use gm::Color;
use render::{rect_view::RectView, ui_rect_instance::UIRectInstance};
use test_engine::{RenderPass, ui::UIImages};
use window::Window;

use crate::pipelines::{IMAGE_DRAWER, UI_RECT};

pub(crate) fn render_occlusion(pass: &mut RenderPass) {
    let rect = UI_RECT.get_mut();
    let image = IMAGE_DRAWER.get_mut();

    rect.add(UIRectInstance::new((50, 50, 50, 50).into(), Color::RED, 0.0, 0.5));

    rect.add(UIRectInstance::new(
        (75, 75, 50, 50).into(),
        Color::GREEN,
        0.0,
        0.5,
    ));
    rect.add(UIRectInstance::new(
        (100, 100, 50, 50).into(),
        Color::BLUE,
        0.0,
        0.5,
    ));

    rect.add(UIRectInstance::new(
        (100, 250, 50, 50).into(),
        Color::BLUE,
        0.0,
        0.5,
    ));
    rect.add(UIRectInstance::new(
        (75, 225, 50, 50).into(),
        Color::GREEN,
        0.0,
        0.5,
    ));
    rect.add(UIRectInstance::new(
        (50, 200, 50, 50).into(),
        Color::RED,
        0.0,
        0.5,
    ));

    rect.add(UIRectInstance::new(
        (50, 350, 50, 50).into(),
        Color::RED,
        0.0,
        0.3,
    ));
    rect.add(UIRectInstance::new(
        (75, 375, 50, 50).into(),
        Color::GREEN,
        0.0,
        0.2,
    ));
    rect.add(UIRectInstance::new(
        (100, 400, 50, 50).into(),
        Color::BLUE,
        0.0,
        0.1,
    ));

    rect.add(UIRectInstance::new(
        (200, 50, 100, 100).into(),
        Color::GREEN,
        0.0,
        0.5,
    ));

    rect.draw(
        pass,
        RectView {
            resolution: Window::inner_size(),
        },
    );

    image.add_with_image(
        UIRectInstance {
            position:      (225, 75).into(),
            size:          (50, 50).into(),
            color:         Default::default(),
            corner_radius: 0.0,
            z_position:    0.4,
        },
        UIImages::rb(),
    );

    image.draw(
        pass,
        RectView {
            resolution: Window::inner_size(),
        },
    );
}
