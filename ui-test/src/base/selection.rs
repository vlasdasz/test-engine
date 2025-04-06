use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Anchor, BLACK, Setup, UI, ViewCallbacks, ViewData, ViewTouch, WHITE, view},
    ui_test::{helpers::check_colors, inject_touches},
};

#[view]
struct Selectable {}

impl Setup for Selectable {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.set_color(BLACK);
    }
}

impl ViewCallbacks for Selectable {
    fn on_selection_changed(&mut self, selected: bool) {
        self.set_color(if selected { WHITE } else { BLACK });
    }
}

#[view]
struct Selection {
    #[init]
    a: Selectable,
    b: Selectable,
    c: Selectable,
}

impl Setup for Selection {
    fn setup(self: Weak<Self>) {
        self.a.place().size(100, 100).center();
        self.b.place().same_size(self.a).center_y().anchor(Anchor::Right, self.a, 40);
        self.c.place().same_size(self.a).center_y().anchor(Anchor::Left, self.a, 40);
    }
}

pub async fn test_selection() -> Result<()> {
    UI::init_test_view::<Selection>().await;

    check_colors(
        r#"
              88  300 -  89 124 149
             174  293 -   0   0   0
             238  303 -  89 124 149
             299  299 -   0   0   0
             375  298 -  89 124 149
             428  296 -   0   0   0
             514  297 -  89 124 149
        "#,
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
        r#"
             163  301 - 255 255 255
             295  308 -   0   0   0
             438  305 -   0   0   0
        "#,
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
        r#"
             160  303 -   0   0   0
             276  308 - 255 255 255
             421  308 -   0   0   0
        "#,
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
        r#"
             180  307 -   0   0   0
             309  303 -   0   0   0
             435  301 - 255 255 255
        "#,
    )
    .await?;

    inject_touches(
        r"
              5    5  b
    ",
    )
    .await;

    check_colors(
        r#"
             186  320 -   0   0   0
             284  312 -   0   0   0
             440  303 -   0   0   0
        "#,
    )
    .await?;

    Ok(())
}
