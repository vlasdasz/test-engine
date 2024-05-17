use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, IntView, Sub, ViewData, ViewSetup, UI},
    ui_test::inject_touches,
    App,
};

#[view]
struct IntTestView {
    int: Sub<IntView>,
}

impl ViewSetup for IntTestView {
    fn setup(self: Weak<Self>) {
        self.int.place().size(50, 150).center();
    }
}

pub async fn test_int_view() -> Result<()> {
    let mut view = UI::init_test_view::<IntTestView>().await;

    App::set_window_size((400, 400));

    assert_eq!(1.0, view.int.value());

    inject_touches(
        "
            215  193  b
            215  193  e
            186  193  b
            186  193  e
            195  198  b
            195  199  e
            208  192  b
            208  192  e
            209  192  b
            209  192  e
            232  190  b
            232  190  e
            189  159  b
            189  159  e
            157  173  b
            156  174  e
            178  192  b
            178  192  e
            208  197  b
            208  197  e
            237  198  b
            237  198  e
            213  198  b
            213  198  e
            157  207  b
            155  207  e
            191  204  b
            192  204  e
            195  196  b
            195  196  e
            195  196  b
            195  196  e
            195  196  b
            195  196  e
            195  196  b
            195  196  e
            195  196  b
            195  196  e
            195  196  b
            195  196  e
            195  196  b
            195  196  e
            195  196  b
            195  196  e
            204  196  b
            204  196  e
            204  196  b
            204  196  e
        ",
    )
    .await;

    assert_eq!(19.0, view.int.value());

    from_main(move || {
        view.int.set_step(4.5);
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
        216.98438    260.84766    b
        186.09766    240.3125     e
        216.98438    260.84766    b
        186.09766    240.3125     e
        216.98438    260.84766    b
        186.09766    240.3125     e
        216.98438    260.84766    b
        186.09766    240.3125     e
        216.98438    260.84766    b
        186.09766    240.3125     e
        216.98438    260.84766    b
        186.09766    240.3125     e
        216.98438    260.84766    b
        186.09766    240.3125     e
        "#,
    )
    .await;

    assert_eq!(-26.0, view.int.value());

    debug!("Int view test: OK");

    Ok(())
}
