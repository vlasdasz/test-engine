use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Anchor, Color, Setup, ViewCallbacks, ViewData, ViewTouch, UI},
    ui_test::{helpers::check_colors, inject_touches},
};

#[view]
struct Selectable {}

impl Setup for Selectable {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.set_color(Color::BLACK);
    }
}

impl ViewCallbacks for Selectable {
    fn on_selection_changed(&mut self, selected: bool) {
        self.set_color(if selected { Color::WHITE } else { Color::BLACK });
    }
}

#[view]
struct SelectionTestView {
    #[init]
    a: Selectable,
    b: Selectable,
    c: Selectable,
}

impl Setup for SelectionTestView {
    fn setup(self: Weak<Self>) {
        self.a.place().size(100, 100).center();
        self.b.place().same_size(self.a).center_y().anchor(Anchor::Right, self.a, 40);
        self.c.place().same_size(self.a).center_y().anchor(Anchor::Left, self.a, 40);
    }
}

pub async fn test_selection() -> Result<()> {
    UI::init_test_view::<SelectionTestView>().await;

    check_colors(
        r"
              84  283 -  25  51  76
             128  274 -   0   0   0
             236  275 -  25  51  76
             280  271 -   0   0   0
             378  272 -  25  51  76
             426  269 -   0   0   0
             535  288 -  25  51  76
    ",
    )
    .await?;

    inject_touches(
        r"
            128  274  b
            128  274  e
    ",
    )
    .await;

    check_colors(
        r"
             140  274 - 255 255 255
             280  271 -   0   0   0
             426  269 -   0   0   0
    ",
    )
    .await?;

    inject_touches(
        r"
            260  260  b
            260  260  e
    ",
    )
    .await;

    check_colors(
        r"
             140  274 -   0   0   0
             280  271 - 255 255 255
             426  269 -   0   0   0
    ",
    )
    .await?;

    inject_touches(
        r"
            420  260  b
            420  260  e
    ",
    )
    .await;

    check_colors(
        r"
             140  274 -   0   0   0
             280  271 -   0   0   0
             426  269 - 255 255 255
    ",
    )
    .await?;

    inject_touches(
        r"
              5    5  b
    ",
    )
    .await;

    check_colors(
        r"
             140  274 -   0   0   0
             280  271 -   0   0   0
             426  269 -   0   0   0
    ",
    )
    .await?;

    debug!("Selection test: OK");

    Ok(())
}
