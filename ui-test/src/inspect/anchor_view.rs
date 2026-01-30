use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    inspect::views::AnchorView,
    refs::Weak,
    ui::{Anchor, Setup, UIDrawer, ViewFrame, view},
    ui_test::check_colors,
};

#[view]
struct AnchorViewTest {
    #[init]
    anchor_view: AnchorView,
}

impl Setup for AnchorViewTest {
    fn setup(mut self: Weak<Self>) {
        self.anchor_view.set_frame((50, 50, 200, 200));
        self.anchor_view.set_anchor(Anchor::Top);
    }
}

pub(crate) async fn test_anchor_view() -> Result<()> {
    let view = UIDrawer::init_test_view::<AnchorViewTest>();

    check_colors(
        r#"
              40   77 -  89 124 149
              98   79 - 255 255 255
              99   59 - 255   0   0
             105   29 -  89 124 149
             222   32 -  89 124 149
             266   48 -  89 124 149
             228   59 - 255   0   0
             234   86 - 255 255 255
             244  162 - 255 255 255
             265  162 -  89 124 149
             105  236 - 255 255 255
              64  194 - 255 255 255
        "#,
    )?;

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Bot);
    });

    check_colors(
        r#"
             132  263 -  89 124 149
             129  242 - 255   0   0
             107  201 - 255 255 255
              30  236 -  89 124 149
             268  238 -  89 124 149
        "#,
    )?;

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Left);
    });

    check_colors(
        r#"
              80  244 - 255 255 255
              59  242 - 255   0   0
             120   57 - 255 255 255
        "#,
    )?;

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Right);
    });

    check_colors(
        r#"
             213   77 - 255 255 255
             241  122 - 255   0   0
             170  241 - 255 255 255
        "#,
    )?;

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Width);
    });

    check_colors(
        r#"
              60  185 - 255 255 255
              78  166 - 255 255 255
              79  150 - 255   0   0
              79  133 - 255 255 255
              69  131 - 255   0   0
              65  114 - 255 255 255
             239  189 - 255 255 255
             238  169 - 255   0   0
             220  168 - 255 255 255
             221  147 - 255   0   0
             225  129 - 255 255 255
             237  111 - 255 255 255
        "#,
    )?;

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Height);
    });

    check_colors(
        r#"
             116   59 - 255 255 255
             130   59 - 255   0   0
             133   83 - 255 255 255
             153   83 - 255   0   0
             169   82 - 255 255 255
             166   58 - 255   0   0
             186   60 - 255 255 255
             184  238 - 255 255 255
             165  238 - 255   0   0
             168  221 - 255 255 255
             152  221 - 255   0   0
             136  221 - 255 255 255
             131  235 - 255   0   0
             112  240 - 255 255 255
        "#,
    )?;

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::MaxWidth);
    });

    check_colors(
        r#"
              88  132 - 255 255 255
             103  150 - 255   0   0
             134  153 -   0   0   0
             141  153 - 255 255 255
             169  132 -   0   0   0
             180  148 - 255 255 255
             208  148 - 255   0   0
        "#,
    )?;

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::MaxHeight);
    });

    check_colors(
        r#"
             129  195 - 255 255 255
             149  197 - 255   0   0
             174  198 - 255 255 255
             149  176 - 255 255 255
             161  141 - 231 231 231
             170  140 - 213 213 213
             148  125 - 255 255 255
             152   95 - 255   0   0
        "#,
    )?;

    Ok(())
}
