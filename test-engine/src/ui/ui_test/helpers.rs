use anyhow::Result;
use gm::{
    color::{Color, LIGHT_GRAY, U8Color},
    flat::Point,
};
use ui::{Button, Container, Setup, UIManager, View, ViewData, ViewSubviews, WeakView};

use crate::{gm::Apply, ui_test::checks::check_colors_structured};

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
    let checks: Vec<_> = data
        .split('\n')
        .filter_map(|line| {
            let parts: Vec<_> = line.split('-').collect();

            if parts.len() != 2 {
                return None;
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

            Some((pos, color))
        })
        .collect();

    check_colors_structured(&checks)
}
