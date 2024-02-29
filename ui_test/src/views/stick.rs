use anyhow::Result;
use log::debug;
use test_engine::{
    refs::{Own, Weak},
    ui::{view, Color, Container, Point, PointView, StickView, SubView, ViewData, ViewFrame, ViewSetup},
    App,
};

use crate::{view_tests::inject_touches, views::image_view::check_colors};

#[view]
struct StickTestView {
    test:  SubView<Container>,
    stick: SubView<StickView>,
    pos:   SubView<PointView>,
    vec:   Own<Point>,
}

impl ViewSetup for StickTestView {
    fn setup(mut self: Weak<Self>) {
        self.test.set_color(Color::GREEN).set_size((50, 50));

        self.stick.set_position((200, 200)).set_size((200, 200));

        self.stick.on_change.val(move |vec| {
            self.test.set_position(vec + 50);
            *self.vec += vec;
        });

        self.pos.set_multiplier(20).place().size(200, 200).bl(0);
        self.pos.changed.val(move |pos| {
            self.stick.set_position(pos);
        });
    }
}

pub async fn test_stick() -> Result<()> {
    let view = App::init_test_view::<StickTestView>(600, 600).await;

    check_colors(
        r#"
             178  231 -  25  51  76
             201  238 -  25  51  76
             221  244 -   0   0   0
             258  250 - 255 255 255
             299  260 - 203 203 203
             351  274 - 255 255 255
             388  281 - 255 255 255
             441  292 -  25  51  76
             442  296 -  25  51  76
             406  300 -  25  51  76
             390  300 - 255 255 255
             347  301 -   0   0   0
             277  309 - 203 203 203
             231  323 - 255 255 255
             190  345 -  25  51  76
             175  359 -  25  51  76
             231  357 - 255 255 255
             255  357 - 255 255 255
             348  374 - 255 255 255
             379  375 -  25  51  76
             403  375 -  25  51  76
             401  374 -  25  51  76
             376  351 - 255 255 255
             374  314 - 255 255 255
             374  246 - 255 255 255
             370  189 -  25  51  76
             329  198 -  25  51  76
             328  236 - 255 255 255
             320  307 - 203 203 203
             315  386 - 255 255 255
             305  415 -  25  51  76
             291  403 -  25  51  76
             261  356 - 255 255 255
             257  329 - 255 255 255
             263  220 - 255 255 255
             259  186 -  25  51  76
             250  214 -   0   0   0
             246  237 - 255 255 255
             235  325 - 255 255 255
             231  386 -  25  51  76
             233  393 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        r#"
            126  352  m
            281  339  m
            367  325  m
            345  310  m
            336  300  b
            336  301  m
            375  351  m
            299  388  m
            180  244  m
            316  153  m
            396  292  m
            212  433  m
            85   166  m
            527  184  m
            187  508  m
            200  165  m
            522  205  m
            205  406  m
            213  167  m
            507  274  m
            212  454  m
            115  205  m
            491  225  m
            232  463  m
            82   213  m
            541  179  m
            264  471  m
            154  182  m
            642  147  m
            307  245  m
            570  210  m
            178  274  m
            361  296  m
            278  301  m
            235  308  m
            550  297  m
            338  309  m
            433  302  m
            374  183  m
            350  324  m
            286  317  m
            236  243  m
            261  253  m
            306  301  m
            342  333  e
            368  322  m
            459  293  m
            476  273  m
            476  272  b
            466  291  m
            440  310  m
            385  308  e
            384  308  m
            340  304  b
            340  304  m
            483  307  m
            516  308  m
            523  308  m
            494  308  e
            344  311  m
            289  292  m
            266  292  m
            293  293  b
            225  295  m
            21   263  m
            -37  251  m
            -8   248  e
            62   244  m
            98   233  m
            91   233  b
            101  236  m
            235  275  m
            321  309  m
            340  277  m
            389  253  m
            104  408  m
            283  321  m
            176  399  m
            213  352  e
            227  346  m
            259  331  m
            268  329  b
            267  330  m
            11   319  m
            123  308  e
            549  342  m
        "#,
    )
    .await;

    assert_eq!(view.vec, Point::new(12.244078, -26.364265));

    debug!("Stick test: OK");

    Ok(())
}
