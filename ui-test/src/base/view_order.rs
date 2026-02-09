use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{BLACK, BLUE, Container, GREEN, RED, Setup, ViewData, ViewSubviews, view},
    ui_test::{UITest, helpers::check_colors},
};

#[view]
pub struct ViewOrder {
    #[init]
    view_1: Container,
    view_2: Container,
    view_3: Container,
    view_4: Container,
}

impl Setup for ViewOrder {
    fn setup(self: Weak<Self>) {
        self.view_1.set_color(RED).place().size(200, 200);
        self.view_2.set_color(GREEN).place().size(200, 200).tl(100);
        self.view_3.set_color(BLUE).place().size(200, 200).tl(200);
        self.view_4.set_color(BLACK).place().size(200, 200).tl(300);
    }
}

pub async fn test_view_order() -> Result<()> {
    let view = UITest::init::<ViewOrder>();

    assert_eq!(
        view.dump_subviews(),
        vec![
            "ViewOrder.view_1: Container".to_string(),
            "ViewOrder.view_2: Container".to_string(),
            "ViewOrder.view_3: Container".to_string(),
            "ViewOrder.view_4: Container".to_string()
        ]
    );

    assert_eq!(view.view_1.view_label(), "ViewOrder.view_1: Container");
    assert_eq!(view.view_2.view_label(), "ViewOrder.view_2: Container");
    assert_eq!(view.view_3.view_label(), "ViewOrder.view_3: Container");
    assert_eq!(view.view_4.view_label(), "ViewOrder.view_4: Container");

    assert_eq!(view.subviews()[0].label(), view.view_1.view_label());
    assert_eq!(view.subviews()[1].label(), view.view_2.view_label());
    assert_eq!(view.subviews()[2].label(), view.view_3.view_label());
    assert_eq!(view.subviews()[3].label(), view.view_4.view_label());

    check_colors(
        r#"
             505  517 -  89 124 149
             497  509 -  89 124 149
             489  502 -  89 124 149
             470  486 -   0   0   0
             443  459 -   0   0   0
             422  445 -   0   0   0
             409  430 -   0   0   0
             380  397 -   0   0   0
             370  387 -   0   0   0
             364  378 -   0   0   0
             362  375 -   0   0   0
             343  349 -   0   0   0
             331  336 -   0   0   0
             321  322 -   0   0   0
             293  294 -   0   0 231
             276  274 -   0   0 231
             239  239 -   0   0 231
             213  223 -   0   0 231
             208  214 -   0   0 231
             176  174 -   0 255   0
             163  155 -   0 255   0
             147  141 -   0 255   0
             140  136 -   0 255   0
             130  125 -   0 255   0
             118  112 -   0 255   0
              94   79 - 255   0   0
              82   60 - 255   0   0
              45   32 - 255   0   0
              44   32 - 255   0   0
        "#,
    )?;

    Ok(())
}
