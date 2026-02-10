use std::any::Any;

use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::{Weak, weak_from_ref},
    ui::{Button, Label, Setup, TableData, TableView, UIManager, ViewData, ViewSubviews, cast_cell, view},
    ui_test::{UITest, check_colors, inject_touches},
};

#[view]
struct ScaleView {
    data: Vec<String>,

    #[init]
    label:  Label,
    button: Button,
    table:  TableView,

    tr_button: Button,
    bl_button: Button,
    br_button: Button,
}

impl Setup for ScaleView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("Label");
        self.label.place().tl(20).size(150, 80);

        self.button.set_text("Button");
        self.button.place().below(self.label, 20);

        self.table.place().size(200, 280).br(20);
        self.table.set_data_source(self);

        self.tr_button.place().tr(20).size(50, 50);
        self.bl_button.place().bl(20).size(50, 50);
        self.br_button.place().br(20).size(50, 50);

        let mut this = self.clone();
        self.apply_to::<Button>(move |b| {
            let b = weak_from_ref(b);
            b.on_tap(move || {
                this.data.push(b.label().to_string());
            });
        });
    }
}

impl TableData for ScaleView {
    fn number_of_cells(self: Weak<Self>) -> usize {
        4
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        cast_cell!(Label).set_text(index);
    }
}

pub async fn test_scale() -> Result<()> {
    let mut view = UITest::start::<ScaleView>();

    inject_touches(
        "
            39   40   b
            39   40   e
            61   541  b
            61   541  e
            538  539  b
            538  539  e
            537  60   b
            537  60   e
            551  83   b
            551  83   e
            517  46   b
            518  46   e
            182  167  b
            182  167  e
            78   171  b
            78   171  e
            82   210  b
            82   210  e
            51   518  b
            51   518  e
            88   560  b
            88   560  e
            516  557  b
            516  557  e

        ",
    );

    let data = from_main(move || view.data.clone());

    assert_eq!(
        data,
        [
            "ScaleView.bl_button: Button",
            "ScaleView.br_button: Button",
            "ScaleView.tr_button: Button",
            "ScaleView.button: Button",
        ]
        .map(ToOwned::to_owned)
        .into_iter()
        .collect::<Vec<_>>()
    );

    check_colors(
        r#"
              55  571 - 255 255 255
              65  572 - 255 255 255
              78  572 -  89 124 149
              85  575 -  89 124 149
              30  539 - 255 255 255
              31  537 - 255 255 255
              34  519 -  89 124 149
              33  511 -  89 124 149
             546  564 - 255 255 255
             532  568 - 255 255 255
             524  570 -  89 124 149
             507  575 -  89 124 149
             553  544 - 255 255 255
             562  539 - 255 255 255
             562  527 -  89 124 149
             561  521 -  89 124 149
             570   59 - 255 255 255
             566   68 - 255 255 255
             566   69 - 255 255 255
             572   85 -  89 124 149
             537   31 - 255 255 255
             537   31 - 255 255 255
             509   25 -  89 124 149
             506   24 -  89 124 149
             418  268 -  89 124 149
             421  306 - 255 255 255
             414  336 - 255 255 255
             365  406 -  89 124 149
             376  444 -  89 124 149
             411  503 -  89 124 149
             426  513 -  89 124 149
             529  503 -  89 124 149
             560  501 -  89 124 149
             573  458 - 255 255 255
             556  265 -  89 124 149
             118  227 -  89 124 149
             130  202 -  89 124 149
             146  161 - 255 255 255
             148  138 - 255 255 255
             139  105 -  89 124 149
             139  105 -  89 124 149
             150   44 - 255 255 255
             148   29 - 255 255 255
             146    1 -  89 124 149
             130    5 -  89 124 149
              34   15 -  89 124 149
               8   53 -  89 124 149
              13   68 -  89 124 149
              41   75 - 255 255 255
             131   82 - 255 255 255
             188   82 -  89 124 149
             179  115 -  89 124 149
             178  128 -  89 124 149
              72  161 -  83  83  83
              63  165 - 175 175 175
              18  193 -  89 124 149
              12  207 -  89 124 149
        "#,
    )?;

    from_main(move || {
        UIManager::override_scale(0.6);
        view.data.clear();
    });

    inject_touches(
        "
            53   958  b
            53   958  e
            53   958  b
            53   958  e
            53   956  b
            53   956  e
            948  942  b
            948  942  e
            955  39   b
            955  39   e
            960  95   b
            960  95   e
            899  52   b
            899  52   e
            128  185  b
            128  185  e

        ",
    );

    let data = from_main(move || view.data.clone());

    assert_eq!(
        data,
        [
            "ScaleView.bl_button: Button",
            "ScaleView.bl_button: Button",
            "ScaleView.bl_button: Button",
            "ScaleView.br_button: Button",
            "ScaleView.tr_button: Button",
            "ScaleView.button: Button",
        ]
        .map(ToOwned::to_owned)
        .into_iter()
        .collect::<Vec<_>>()
    );

    check_colors(
        r#"
             537  577 -  89 124 149
             552  575 -  89 124 149
             564  573 - 255 255 255
             574  553 -  89 124 149
             569  529 - 255 255 255
             555  455 - 255 255 255
             559  410 -  89 124 149
             545  383 -  89 124 149
             478  400 -  89 124 149
             438  446 -  89 124 149
             484  484 - 255 255 255
             479  555 -  89 124 149
             568   59 -  89 124 149
             571   40 - 255 255 255
             570   25 - 255 255 255
             557   21 -  89 124 149
             537   16 -  89 124 149
              35  527 -  89 124 149
              31  550 -  89 124 149
              30  576 - 255 255 255
              50  575 -  89 124 149
              61  567 -  89 124 149
              31  133 -  89 124 149
              32  131 -  89 124 149
              33  113 - 255 255 255
              34  102 - 255 255 255
              34   80 - 255 255 255
              32   64 -  89 124 149
              32   57 - 255 255 255
              31   37 - 255 255 255
              32   24 - 255 255 255
              62   21 - 255 255 255
              72   22 - 255 255 255
              91   26 - 255 255 255
             114   27 -  89 124 149
             143   29 -  89 124 149
             128   58 -  89 124 149
             106   94 -  89 124 149
              91  107 - 255 255 255
              87  107 - 255 255 255
               6   89 -  89 124 149
              21   88 - 255 255 255
               6   34 -  89 124 149
              14   39 - 255 255 255
              31   45 - 255 255 255
        "#,
    )?;

    from_main(move || {
        UIManager::override_scale(1.5);
        view.data.clear();
    });

    inject_touches(
        "
            40   389  b
            40   389  e
            44   363  b
            44   363  e
            40   318  b
            40   318  e
            307  356  b
            308  356  e
            347  356  b
            347  356  e
            390  355  b
            390  355  e
            348  86   b
            348  86   e
            352  45   b
            352  45   e
            349  10   b
            349  10   e
            75   112  b
            75   112  e
            74   135  b
            74   135  e
            63   185  b
            63   185  e
            59   215  b
            59   215  e

        ",
    );

    let data = from_main(move || view.data.clone());

    assert_eq!(
        data,
        [
            "ScaleView.bl_button: Button",
            "ScaleView.br_button: Button",
            "ScaleView.tr_button: Button",
            "ScaleView.button: Button",
            "ScaleView.button: Button"
        ]
        .map(ToOwned::to_owned)
        .into_iter()
        .collect::<Vec<_>>()
    );

    check_colors(
        r#"
              18  523 -  89 124 149
              46  513 - 255 255 255
              98  513 - 255 255 255
             111  516 -  89 124 149
             474  513 -  89 124 149
             514  514 - 255 255 255
             563  515 - 255 255 255
             587  512 -  89 124 149
             557  575 -  89 124 149
             558  543 - 255 255 255
             555  472 -  89 124 149
             531  454 -  89 124 149
             509  390 - 255 255 255
             476  287 - 255 255 255
             447  213 - 255 255 255
             417  125 -  89 124 149
             351  121 -  89 124 149
             285  196 - 255 255 255
             271  216 - 255 255 255
             249  218 - 255 255 255
             205  220 - 255 255 255
             116  223 - 255 255 255
              16  225 -  89 124 149
              51  125 - 255 255 255
              16   97 -  89 124 149
              79   14 -  89 124 149
             145   57 - 255 255 255
             201  159 -  89 124 149
             219  109 - 255 255 255
             326   93 -  89 124 149
             486   86 -  89 124 149
             511   82 - 255 255 255
             583   84 -  89 124 149
             574  193 -  89 124 149
             538  249 - 255 255 255
             394  352 - 255 255 255
             256  409 -  89 124 149
             193  419 -  89 124 149
             181  278 - 255 255 255
             220  183 - 255 255 255
        "#,
    )?;

    from_main(move || {
        UIManager::override_scale(1);
    });

    Ok(())
}
