use anyhow::Result;
use test_engine::{
    gm::Apply,
    refs::Weak,
    ui::{
        Anchor::{Left, Size, Top, X},
        Color, Container, ImageView, Setup, UI, ViewData, view,
    },
    ui_test::check_colors,
};

#[view]
struct Colors {
    #[init]
    image: ImageView,

    _1: Container,
    _2: Container,
    _3: Container,
    _4: Container,
}

impl Setup for Colors {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::WHITE);

        self.image.place().tl(20).size(280, 520);
        self.image.set_image("colors.png");

        self._1.set_color((45, 70, 149));
        self._1.place().size(100, 100).t(45).anchor(Left, self.image, 20);

        [self._2, self._3, self._4].apply(|view| {
            view.place().same([Size, X], self._1);
        });

        self._2.set_color((48, 48, 48));
        self._2.place().anchor(Top, self._1, 25);

        self._3.set_color((124, 190, 22));
        self._3.place().anchor(Top, self._2, 25);

        self._4.set_color((172, 71, 212));
        self._4.place().anchor(Top, self._3, 25);
    }
}

pub async fn test_colors() -> Result<()> {
    UI::init_test_view::<Colors>().await;

    check_colors(
        r#"
             249   81 -  45  70 149
             385   96 -  45  70 149
             238  224 -  48  48  48
             370  225 -  48  48  48
             252  357 - 124 190  22
             379  342 - 124 190  22
             222  477 - 172  71 212
             374  477 - 172  71 212
        "#,
    )
    .await?;

    Ok(())
}
