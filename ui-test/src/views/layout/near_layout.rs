use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{BLUE, Container, GREEN, Setup, ViewData, view},
    ui_test::{UITest, check_colors},
};

#[view]
struct NearLayout {
    #[init]
    base: Container,
    next: Container,
}

impl Setup for NearLayout {
    fn setup(self: Weak<Self>) {
        self.base.set_color(GREEN);
        self.next.set_color(BLUE);

        self.base.place().tl(20).size(50, 80);

        self.next.place().at_right(self.base, 20);
    }
}

pub async fn test_near_layout() -> anyhow::Result<()> {
    let view = UITest::init::<NearLayout>();

    check_colors(
        r#"
                  13   62 -  89 124 149
                  36   62 -   0 255   0
                  63   63 -   0 255   0
                  74   63 -  89 124 149
                  83   62 -  89 124 149
                  99   62 -   0   0 231
                 130   66 -   0   0 231
                 160   69 -  89 124 149
                 114  109 -  89 124 149
                 114   91 -   0   0 231
                 109   15 -  89 124 149
                 102   30 -   0   0 231
                  46   14 -  89 124 149
                  41   35 -   0 255   0
                  43  106 -  89 124 149
                  46   94 -   0 255   0
            "#,
    )?;

    from_main(move || {
        view.next.place().clear().below(view.base, 20);
    });

    check_colors(
        r#"
                  44  209 -  89 124 149
                  44  193 -   0   0 231
                  44  128 -   0   0 231
                  45  118 -  89 124 149
                  45   98 -   0 255   0
                  45   13 -  89 124 149
            "#,
    )?;

    from_main(move || {
        view.next.place().clear().at_right(view.base, 20).w(200);
    });

    check_colors(
        r#"
                  14   93 -  89 124 149
                  52   94 -   0 255   0
                  87   92 -  89 124 149
                 100   91 -   0   0 231
                 160   91 -   0   0 231
                 222   89 -   0   0 231
                 284   89 -   0   0 231
                 298   89 -  89 124 149
            "#,
    )?;

    from_main(move || {
        view.next.place().clear().below(view.base, 20).w(200);
    });

    check_colors(
        r#"
                  49   95 -   0 255   0
                  48  114 -  89 124 149
                  11  151 -  89 124 149
                  33  154 -   0   0 231
                  38  211 -  89 124 149
                  99  179 -   0   0 231
                 139  109 -  89 124 149
                 139  136 -   0   0 231
                 201  112 -  89 124 149
                 228  134 -  89 124 149
                 209  158 -   0   0 231
                 226  173 -  89 124 149
                 194  191 -   0   0 231
                 183  210 -  89 124 149
            "#,
    )?;

    from_main(move || {
        view.next.place().clear().below(view.base, 20).h(10);
    });

    check_colors(
        r#"
                  33  137 -  89 124 149
                  33  127 -   0   0 231
                  36  116 -  89 124 149
            "#,
    )?;

    Ok(())
}
