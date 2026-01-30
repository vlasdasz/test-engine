use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::{Own, Weak},
    ui::{Container, GREEN, Point, PointView, Setup, UIDrawer, ViewData, ViewFrame, view},
    ui_test::inject_touches,
};

#[view]
struct StickView {
    vec: Own<Point>,

    #[init]
    test:  Container,
    stick: test_engine::ui::StickView,
    pos:   PointView,
}

impl Setup for StickView {
    fn setup(mut self: Weak<Self>) {
        self.test.set_color(GREEN).set_size(50, 50);

        self.stick.set_position((200, 200)).set_size(200, 200);

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
    let mut view = UIDrawer::init_test_view::<StickView>();

    inject_touches(
        r"
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
        ",
    );

    assert_eq!(view.vec, Point::new(12.244_078, -26.364_265));

    from_main(move || {
        view.stick.set_position((400, 50));
    });
    //
    // check_colors(
    //     r#"
    //          403  108 -  89 124 149
    //          405  108 -  89 124 149
    //          410  110 -  89 124 149
    //          419  115 -  89 124 149
    //          429  120 - 112 112 112
    //          431  122 -  89 124 149
    //          445  131 -  89 124 149
    //          465  143 - 195 195 195
    //          468  146 - 195 195 195
    //          473  147 - 195 195 195
    //          477  152 - 195 195 195
    //          480  152 - 195 195 195
    //          497  157 - 195 195 195
    //          535  169 - 112 112 112
    //          511  173 - 195 195 195
    //          513  173 - 195 195 195
    //          520  183 - 194 194 194
    //          524  191 - 112 112 112
    //          548  215 - 112 112 112
    //          556  222 -  89 124 149
    //          554  234 -  89 124 149
    //          559   79 -  89 124 149
    //          551   87 - 112 112 112
    //          549   90 - 112 112 112
    //          537  109 -  89 124 149
    //          525  120 - 195 195 195
    //          517  131 - 195 195 195
    //          496  165 - 195 195 195
    //          470  179 - 112 112 112
    //          461  188 -  89 124 149
    //          448  203 -  89 124 149
    //          443  209 - 112 112 112
    //          431  231 -  89 124 149
    //          426  237 -  89 124 149
    //     "#,
    // )
    // .await?;

    from_main(move || {
        view.stick.set_position((200, 200)).set_size(400, 400);
    });

    // UIManager::enable_debug_frames();

    // record_ui_test().await;

    Ok(())
}
