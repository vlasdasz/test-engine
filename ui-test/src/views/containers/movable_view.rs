use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Container, GREEN, MovableView, Setup, UIDrawer, ViewData, ViewFrame, view},
    ui_test::{check_colors, inject_touches},
};

#[view]
struct MovableViewTestView {
    #[init]
    movable: MovableView<Container>,
}

impl Setup for MovableViewTestView {
    fn setup(mut self: Weak<Self>) {
        self.movable.set_title("Movable view");
        self.movable.set_frame((10, 10, 400, 400));
        self.movable.target_view.set_color(GREEN);
    }
}

pub async fn test_movable_view() -> Result<()> {
    let mut _view = UIDrawer::init_test_view::<MovableViewTestView>();

    inject_touches(
        "
            346  36   b
            438  90   m
            438  90   e
        ",
    );

    check_colors(
        r#"
             528  425 -  89 124 149
             486  425 -   0 255   0
             476  425 -   0 255   0
             442  426 -   0 255   0
             383  434 -   0 255   0
             359  449 -   0 255   0
             255  485 -  89 124 149
              57  388 -  89 124 149
             107  377 -   0 255   0
             107  377 -   0 255   0
             109  336 -   0 255   0
             177  244 -   0 255   0
             293  177 -   0 255   0
             174   39 -  89 124 149
             174   68 - 255 255 255
             174  110 -   0 255   0
             510  132 -  89 124 149
             493   83 - 255 255 255
        "#,
    )?;

    inject_touches(
        "
            501  458  b
            323  192  m
            323  192  e
        ",
    );

    check_colors(
        r#"
             207  232 -  89 124 149
             207  182 -   0 255   0
              93  153 -  89 124 149
             123  153 -   0 255   0
             125   43 -  89 124 149
             133  116 -   0 255   0
             357  125 -  89 124 149
             287  125 -   0 255   0
             324  217 -  89 124 149
             255  157 -   0 255   0
        "#,
    )?;

    inject_touches(
        "
            313  190  b
            78   78   m
            78   78   e
        ",
    );

    check_colors(
        r#"
             165  177 -  89 124 149
             155  149 -   0 255   0
              82  138 -  89 124 149
             113  137 -   0 255   0
             183  121 -   0 255   0
             209  121 -  89 124 149
             153   51 -  89 124 149
             152  127 -   0 255   0
             146  168 -  89 124 149
             126  175 -  89 124 149
              97  148 -  89 124 149
             134  160 -   0 255   0
        "#,
    )?;

    Ok(())
}
