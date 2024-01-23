use anyhow::Result;
use log::debug;
use test_engine::{from_main, gm::flat::IntSize, Screen};
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::IntView;

use crate::view_tests::{assert_eq, inject_touches};

#[view]
struct IntTestView {
    int: SubView<IntView>,
}

impl ViewSetup for IntTestView {
    fn setup(self: Weak<Self>) {
        self.int.place.size(50, 150).center();
    }
}

impl ViewTest for IntTestView {
    fn test_size() -> IntSize
    where Self: Sized {
        (400, 400).into()
    }
}

pub async fn test_int_view() -> Result<()> {
    Screen::set_test_view::<IntTestView>(400, 400).await;

    let mut this = IntTestView::instance();

    assert_eq(1.0, this.int.value())?;

    inject_touches(
        r#"
        218.80078    217.51563    b
        218.80078    217.51563    e
        187.6289     216.59375    b
        187.6289     216.59375    e
        129.71094    183.92188    b
        129.71094    183.92188    e
        199.58984    181.66797    b
        199.58984    181.66797    e
        210.6211     140.90625    b
        210.8711     140.65234    e
        304.35938    171.07813    b
        304.35938    171.32813    e
        204.40234    199.19922    b
        204.40234    199.19922    e
        "#,
    )
    .await;

    assert_eq(5.0, this.int.value())?;

    from_main(move || {
        this.int.set_step(4.5);
    })
    .await;

    inject_touches(
        r#"
        217.51563    267.8711     b
        193.09766    247.10156    e
        206.96484    234.54297    b
        193.39063    258.7539     e
        203.41016    199.65625    b
        192.01172    191.97266    e
        190.83594    258.30078    b
        216.72656    228.10156    e
        216.98438    260.84766    b
        186.09766    240.3125     e
        "#,
    )
    .await;

    assert_eq(-8.5, this.int.value())?;

    debug!("Int view test: OK");

    Ok(())
}
