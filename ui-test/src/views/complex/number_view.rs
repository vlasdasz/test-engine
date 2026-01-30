use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        Anchor::{Bot, Height, Left, Right, Width, X, Y},
        HasText, Label, NumberView, Setup, UIDrawer, ViewData, view,
    },
    ui_test::inject_touches,
};

#[view]
struct NumberTestView {
    #[init]
    float: NumberView,
    uint:  NumberView,
    int:   NumberView,

    float_label: Label,
    uint_label:  Label,
    int_label:   Label,
}

impl Setup for NumberTestView {
    fn setup(self: Weak<Self>) {
        fn attach_label(mut label: Weak<Label>, view: Weak<NumberView>) {
            label.place().same([Width, X], view).h(50).anchor(Bot, view, 20);
            view.on_change(move |num| {
                label.set_text(num);
            });
        }

        self.float.place().tl(200).size(100, 200);
        attach_label(self.float_label, self.float);

        self.uint
            .place()
            .same([Width, Height, Y], self.float)
            .anchor(Left, self.float, 20);
        attach_label(self.uint_label, self.uint);

        self.int
            .place()
            .same([Width, Height, Y], self.float)
            .anchor(Right, self.float, 20);
        attach_label(self.int_label, self.int);
    }
}

pub async fn test_number_view() -> Result<()> {
    let mut view = UIDrawer::init_test_view::<NumberTestView>();

    inject_touches(
        "
            379  244  b
            378  243  e
            378  243  b
            378  243  e
            378  243  b
            378  243  e
            378  243  b
            378  243  e
            378  243  b
            378  243  e
            378  243  b
            378  243  e
            378  243  b
            378  243  e
            248  371  b
            248  371  e
            248  371  b
            248  371  e
            248  371  b
            248  371  e
            248  371  b
            248  371  e
            248  371  b
            248  371  e
            248  371  b
            248  371  e
            248  371  b
            248  371  e
            112  234  b
            112  234  e
            112  234  b
            112  234  e
            112  234  b
            112  234  e
            112  234  b
            112  234  e
            112  234  b
            112  234  e
            112  234  b
            112  234  e
            112  234  b
            112  234  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            124  352  b
            124  352  e
            356  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            355  334  b
            355  334  e
            361  247  b
            361  247  e
            361  247  b
            361  247  e
            361  247  b
            361  247  e
            361  247  b
            361  247  e
            361  247  b
            361  247  e
            361  247  b
            361  247  e
            361  247  b
            361  247  e
            361  325  b
            361  325  e

        ",
    );

    from_main(move || {
        assert_eq!(view.float.value(), -6.0);
        assert_eq!(view.uint.value(), 3.0);
        assert_eq!(view.int.value(), -6.0);

        view.float.set_min(-10.0);
        view.uint.set_min(2);
        view.int.set_min(-10);

        assert_eq!(view.float.value(), -10.0);
        assert_eq!(view.uint.value(), 2.0);
        assert_eq!(view.int.value(), -10.0);
    });

    inject_touches(
        "
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            160  266  b
            160  266  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            230  255  b
            230  255  e
            285  345  b
            285  345  e
            363  234  b
            363  234  e
            363  234  b
            363  234  e
            363  234  b
            363  234  e

        ",
    );

    assert_eq!(view.float.value(), 5.0);
    assert_eq!(view.uint.value(), 5.0);
    assert_eq!(view.int.value(), 5.0);

    inject_touches(
        "
            126  344  b
            126  344  e
            126  344  b
            126  344  e
            126  344  b
            126  344  e
            126  344  b
            126  344  e
            126  344  b
            126  344  e
            126  349  b
            126  349  e
            126  349  b
            126  349  e
            126  349  b
            126  349  e
            126  349  b
            126  349  e
            126  349  b
            126  349  e
            126  344  b
            126  344  e
            126  344  b
            126  344  e
            126  344  b
            126  344  e
            126  344  b
            126  344  e
            126  342  b
            126  342  e
            126  342  b
            126  342  e
            126  342  b
            126  342  e
            126  342  b
            126  342  e
            126  342  b
            126  342  e
            126  342  b
            126  342  e
            126  340  b
            126  340  e
            126  340  b
            126  340  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            126  339  b
            126  339  e
            257  374  b
            257  374  e
            257  374  b
            257  374  e
            257  374  b
            257  374  e
            257  374  b
            257  374  e
            257  374  b
            257  374  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  369  b
            257  369  e
            257  368  b
            257  368  e
            257  368  b
            257  368  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            257  366  b
            257  366  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e
            386  353  b
            386  353  e

        ",
    );

    assert_eq!(view.float.value(), -10.0);
    assert_eq!(view.uint.value(), 2.0);
    assert_eq!(view.int.value(), -10.0);

    Ok(())
}
