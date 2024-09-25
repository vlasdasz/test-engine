use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, NumberView, Setup, ViewData, UI},
    ui_test::inject_touches,
    App,
};

#[view]
struct NumberTestView {
    #[init]
    float: NumberView<f32>,
    uint:  NumberView<u32>,
    int:   NumberView<i32>,
}

impl Setup for NumberTestView {
    fn setup(self: Weak<Self>) {
        self.place().all_hor();
    }
}

pub async fn test_number_view() -> Result<()> {
    let mut view = UI::init_test_view::<NumberTestView>().await;

    App::set_window_size((400, 400)).await;

    inject_touches(
        "
            86   205  b
            86   205  e
            83   202  b
            83   202  e
            83   202  b
            83   202  e
            83   202  b
            84   202  e
            84   202  b
            84   202  e
            84   202  b
            84   202  e
            84   202  b
            84   202  e
            84   202  b
            84   202  e
            84   202  b
            84   202  e
            84   202  b
            84   202  e
            209  193  b
            210  193  e
            210  193  b
            210  193  e
            210  193  b
            210  193  e
            210  193  b
            210  193  e
            210  193  b
            210  193  e
            210  193  b
            210  193  e
            210  193  b
            210  193  e
            210  193  b
            210  193  e
            209  193  b
            209  193  e
            209  193  b
            209  193  e
            210  193  b
            210  193  e
            210  193  b
            210  193  e
            210  193  b
            217  191  e
            320  187  b
            321  187  e
            321  187  b
            321  187  e
            321  187  b
            321  187  e
            321  187  b
            321  187  e
            321  187  b
            324  188  e
            324  189  b
            324  189  e
            325  189  b
            325  189  e
            325  189  b
            325  189  e
            325  189  b
            325  189  e
            325  189  b
            325  189  e
            325  189  b
            325  189  e
            326  189  b
            326  189  e
            326  189  b
            326  189  e
            326  189  b
            326  189  e
            326  189  b
            326  189  e
            346  349  b
            346  349  e
            346  349  b
            346  349  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            346  348  b
            346  348  e
            345  348  b
            345  348  e
            343  348  b
            343  348  e
            343  348  b
            343  348  e
            343  348  b
            343  348  e
            342  349  b
            342  349  e
            338  349  b
            338  349  e
            338  349  b
            338  349  e
            336  349  b
            336  349  e
            336  349  b
            336  349  e
            336  349  b
            336  349  e
            346  236  b
            346  236  e
            204  338  b
            205  338  e
            205  338  b
            205  338  e
            205  338  b
            205  338  e
            205  338  b
            205  338  e
            205  338  b
            205  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            204  338  b
            204  338  e
            226  192  b
            226  192  e
            192  195  b
            192  195  e
            192  195  b
            198  209  e
            235  315  b
            235  315  e
            230  320  b
            230  320  e
            223  327  b
            221  329  e
            209  334  b
            209  335  e
            108  345  b
            108  345  e
            108  345  b
            108  345  e
            104  344  b
            104  344  e
            103  344  b
            100  344  e
            97   344  b
            96   344  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            86   342  b
            86   342  e
            85   342  b
            85   342  e
            85   342  b
            85   342  e
            85   342  b
            85   341  e
            85   341  b
            85   341  e
            85   341  b
            85   341  e
            85   341  b
            85   341  e
        ",
    )
    .await;

    from_main(move || {
        assert_eq!(view.float.value(), -10.0);
        assert_eq!(view.uint.value(), 0);
        assert_eq!(view.int.value(), -10);

        view.float.set_min(-5.0);
        view.uint.set_min(10);
        view.int.set_min(-5);

        assert_eq!(view.float.value(), -5.0);
        assert_eq!(view.uint.value(), 10);
        assert_eq!(view.int.value(), -5);
    })
    .await;

    inject_touches(
        "
            82   202  b
            82   202  e
            81   202  b
            81   202  e
            81   201  b
            81   201  e
            81   201  b
            81   201  e
            81   201  b
            81   201  e
            182  202  b
            182  201  e
            182  201  b
            182  201  e
            182  201  b
            182  201  e
            182  201  b
            182  201  e
            182  201  b
            182  201  e
            344  182  b
            344  182  e
            344  182  b
            343  182  e
            343  182  b
            343  182  e
            343  182  b
            343  182  e
            343  182  b
            343  182  e
        ",
    )
    .await;

    assert_eq!(view.float.value(), 0.0);
    assert_eq!(view.uint.value(), 15);
    assert_eq!(view.int.value(), 0);

    inject_touches(
        "
            72   337  b
            72   337  e
            72   337  b
            72   337  e
            70   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            69   337  b
            69   337  e
            210  331  b
            210  331  e
            209  331  b
            209  331  e
            208  331  b
            208  331  e
            207  331  b
            207  330  e
            206  330  b
            206  330  e
            206  330  b
            206  330  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            203  329  b
            203  329  e
            352  322  b
            352  322  e
            349  322  b
            349  322  e
            349  322  b
            349  322  e
            349  322  b
            348  322  e
            348  322  b
            348  322  e
            347  322  b
            347  322  e
            347  322  b
            347  322  e
            345  323  b
            345  323  e
            345  323  b
            345  323  e
            343  323  b
            343  323  e
            343  323  b
            343  323  e
            342  323  b
            342  323  e
            342  323  b
            342  323  e
            342  323  b
            342  323  e
            342  323  b
            342  323  e
            341  323  b
            341  323  e
            314  199  b
            314  199  e
            331  344  b
            331  344  e
            331  343  b
            331  343  e
            194  187  b
            194  187  e
            187  329  b
            187  329  e
            187  329  b
            187  329  e
            62   193  b
            62   193  e
            70   331  b
            70   331  e
            70   331  b
            70   331  e
        ",
    )
    .await;

    assert_eq!(view.float.value(), -5.0);
    assert_eq!(view.uint.value(), 10);
    assert_eq!(view.int.value(), -5);

    debug!("Number view test: OK");

    Ok(())
}
