use std::{fmt::Display, sync::OnceLock};

use anyhow::{bail, Result};
use log::{debug, error};
use test_engine::{from_main, gm::flat::IntSize, Screen};
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::IntView;

use crate::view_tests::inject_touches;

static THIS: OnceLock<Weak<IntTestView>> = OnceLock::new();

#[view]
struct IntTestView {
    int: SubView<IntView>,
}

impl ViewSetup for IntTestView {
    fn setup(self: Weak<Self>) {
        THIS.set(self).unwrap();
        self.int.place.back().size(50, 150).center();
    }
}

impl ViewTest for IntTestView {
    fn test_size() -> IntSize
    where Self: Sized {
        (400, 400).into()
    }
}

fn assert_eq<T: PartialEq + Display>(a: T, b: T) -> Result<()> {
    if a == b {
        return Ok(());
    }
    let message = format!("Assertion failed: {a} != {b}");
    error!("{message}");
    bail!(message)
}

pub async fn int_view_test() -> Result<()> {
    Screen::set_test_view::<IntTestView>().await;

    let mut this = THIS.get().unwrap().clone();

    assert_eq(1.0, this.int.value())?;

    inject_touches(
        r#"
        218.80078    217.51563    ↓
        218.80078    217.51563    ↑
        187.6289     216.59375    ↓
        187.6289     216.59375    ↑
        129.71094    183.92188    ↓
        129.71094    183.92188    ↑
        199.58984    181.66797    ↓
        199.58984    181.66797    ↑
        210.6211     140.90625    ↓
        210.8711     140.65234    ↑
        304.35938    171.07813    ↓
        304.35938    171.32813    ↑
        204.40234    199.19922    ↓
        204.40234    199.19922    ↑
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
        217.51563    267.8711     ↓
        193.09766    247.10156    ↑
        206.96484    234.54297    ↓
        193.39063    258.7539     ↑
        203.41016    199.65625    ↓
        192.01172    191.97266    ↑
        190.83594    258.30078    ↓
        216.72656    228.10156    ↑
        216.98438    260.84766    ↓
        186.09766    240.3125     ↑
        "#,
    )
    .await;

    assert_eq(-8.5, this.int.value())?;

    debug!("Int view test: OK");

    Ok(())
}
