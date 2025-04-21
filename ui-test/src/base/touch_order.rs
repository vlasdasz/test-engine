use anyhow::Result;
use test_engine::{
    gm::Apply,
    refs::Weak,
    ui::{BLACK, BLUE, Container, GREEN, RED, Setup, TouchStack, UI, ViewData, ViewTouch, view},
    ui_test::{
        inject_touches,
        state::{append_state, clear_state, get_state},
    },
};

#[view]
struct TouchOrder {
    #[init]
    view_1: Container,
    view_2: Container,
    view_3: Container,
    view_4: Container,
}

impl Setup for TouchOrder {
    fn setup(mut self: Weak<Self>) {
        self.view_1.set_color(RED);
        self.view_2.set_color(GREEN).place().tl(50);
        self.view_3.set_color(BLUE).place().tl(100);
        self.view_4.set_color(BLACK).place().tl(150);

        [self.view_1, self.view_2, self.view_3, self.view_4].apply(|mut v| {
            v.enable_touch().place().size(200, 200);
            let color = v.color().with_alpha(0.5);
            v.set_color(color);
            v.touch().up_inside.sub(move || {
                append_state(&format!("{}\n", v.view_label()));
            });
        });
    }
}

pub async fn test_touch_order() -> Result<()> {
    UI::init_test_view::<TouchOrder>().await;

    assert_eq!(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            "TouchOrder.view_1: Container".to_string(),
            "TouchOrder.view_2: Container".to_string(),
            "TouchOrder.view_3: Container".to_string(),
            "TouchOrder.view_4: Container".to_string(),
        ]],
    );

    clear_state();

    inject_touches(
        r"
            376  385  b
            373  383  e
            310  331  b
            310  331  e
            274  277  b
            272  277  e
            213  226  b
            210  224  e
            185  175  b
            185  175  e
            129  130  b
            129  130  e
            74   87   b
            74   87   e
            29   48   b
            29   48   e
        ",
    )
    .await;

    assert_eq!(
        get_state::<String>(),
        r"TouchOrder.view_4: Container
TouchOrder.view_4: Container
TouchOrder.view_4: Container
TouchOrder.view_4: Container
TouchOrder.view_3: Container
TouchOrder.view_2: Container
TouchOrder.view_1: Container
"
    );

    Ok(())
}
