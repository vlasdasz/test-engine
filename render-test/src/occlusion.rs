use gm::{Color, flat::Size};
use render::{rect_view::RectView, ui_rect_instance::UIRectInstance};
use test_engine::{RenderPass, ui::UIImages};
use window::Window;

use crate::pipelines::{IMAGE_DRAWER, UI_RECT};

pub(crate) fn render_occlusion(pass: &mut RenderPass) {
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
}
