use anyhow::Result;
use gm::{
    color::{Color, LIGHT_GRAY, U8Color},
    flat::Point,
};
use hreads::from_main;
use ui::{
    Button, Container, HighlightView, Setup, UIManager, View, ViewData, ViewFrame, ViewSubviews, WeakView,
};

use crate::{
    AppRunner,
    gm::Apply,
    ui::{Screenshot, ui::TEST_NAME},
};

pub fn add_corners(mut view: WeakView, color: Color) {
    let v1 = view.add_view::<Container>();
    let v2 = view.add_view::<Container>();
    let v3 = view.add_view::<Container>();
    let v4 = view.add_view::<Container>();

    [v1, v2, v3, v4].apply(|mut a| {
        a.place().size(100, 100);
        a.set_color(color);
    });

    v1.place().tl(0);
    v2.place().tr(0);
    v3.place().bl(0);
    v4.place().br(0);
}

#[allow(dead_code)]
pub fn add_action(action: impl FnMut() + Send + 'static) {
    let mut button = UIManager::root_view()
        .add_subview_to_root(Button::new())
        .downcast::<Button>()
        .unwrap();
    button.place().size(100, 100).bl(0);
    button.set_color(LIGHT_GRAY);
    button.on_tap(action);
    button.base_view_mut().view_label = "Debug Action Button".into();
}

pub fn check_colors(data: &str) -> Result<()> {
    let screenshot = AppRunner::take_screenshot()?;

    let lines: Vec<_> = data.split('\n').collect();

    for line in lines {
        let parts: Vec<_> = line.split('-').collect();

        if parts.len() != 2 {
            continue;
        }

        let pos = parts[0];
        let color = parts[1];

        let pos: Vec<_> = pos.split(' ').filter(|a| !a.is_empty()).collect();
        let color: Vec<_> = color.split(' ').filter(|a| !a.is_empty()).collect();

        let pos: Point = Point::new(pos[0].parse().unwrap(), pos[1].parse().unwrap());
        let color: U8Color = U8Color::rgba(
            color[0].parse().unwrap(),
            color[1].parse().unwrap(),
            color[2].parse().unwrap(),
            255,
        );

        check_pixel_color(&screenshot, pos, color);
    }

    Ok(())
}

pub fn check_pixel_color(screenshot: &Screenshot, pos: Point, color: U8Color) {
    let pixel: U8Color = screenshot.get_pixel(pos);

    let diff = pixel.diff_u8(color);

    let max_diff = 45;

    if diff > max_diff {
        from_main(move || {
            let mut high = HighlightView::new();
            high.set_z_position(0.1);

            UIManager::root_view()
                .add_subview_to_root(high)
                .downcast_view::<HighlightView>()
                .unwrap()
                .set(pos, color.into(), pixel.into());
        });
    }

    let test_name = TEST_NAME.lock().clone();

    assert!(
        diff <= max_diff,
        r"
        Test: {test_name} has failed.
        Color diff is too big: {diff}. Max: {max_diff}. Position: {pos:?}.
        Expected: {color}, got: {pixel}.
        {:>4} {:>4} - {:>3} {:>3} {:>3}",
        pos.x,
        pos.y,
        color.r,
        color.g,
        color.b
    );
}
