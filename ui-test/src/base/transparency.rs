use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    gm::Apply,
    level::LevelManager,
    refs::Weak,
    ui::{ImageView, Setup, UIDrawer, ViewData, view},
    ui_test::check_colors,
};

use crate::level::SkyboxLevel;

#[view]
struct Transparency {
    #[init]
    background: ImageView,

    view_1: ImageView,
    view_2: ImageView,
    view_3: ImageView,
    view_4: ImageView,
}

impl Setup for Transparency {
    fn setup(mut self: Weak<Self>) {
        self.background.set_image("gradient.png").place().back();

        self.view_1.set_image("wood-window.png");
        self.view_2.set_image("wood-window.png").place().tl(50);
        self.view_3.set_image("wood-window.png").place().tl(100);
        self.view_4.set_image("wood-window.png").place().tl(150);

        [self.view_1, self.view_2, self.view_3, self.view_4].apply(|v| {
            v.place().size(280, 280);
        });
    }
}

pub async fn test_transparency() -> Result<()> {
    UIDrawer::init_test_view::<Transparency>();

    from_main(|| {
        LevelManager::set_level(SkyboxLevel::default());
    });

    from_main(|| {
        LevelManager::stop_level();
    });

    check_colors(
        r#"
              19  396 - 157  91  32
              30  395 - 157  92  33
              45  388 -  89 124 149
              58  380 -  89 124 149
             101  341 -  89 124 149
             126  334 - 211 164 120
             158  320 - 173 125  87
             159  317 - 181 143 107
             266  262 -   9  10  12
             189  262 - 178 126  79
             191  262 - 175 119  86
             208  164 - 102  49  18
             209   89 - 125  76  43
             209   84 - 170 108  57
             227   78 - 185 133  86
             228   76 - 223 182 138
             228   76 - 223 182 138
             200   67 - 241 210 182
             192   63 -  26  21  17
             135   15 -  27  21  21
             110   19 - 197 153 114
             104   21 - 240 222 200
              90   30 - 144 113  85
              86   35 - 183 141  99
              80   41 -   7   0   7
              69   46 - 130  77  46
              69   48 - 123  82  64
              69   53 - 127  96  68
              66   69 - 120  72  50
             198  105 - 224 182 134
             199  105 - 233 191 143
             206  116 - 125  74  45
             207  119 - 123  87  73
             220  127 - 179 143  85
             233  191 -  21   3   3
             236  193 - 140  86  58
             244  260 -  89 124 149
             246  283 -  89 124 149
             248  287 - 180 129  86
             249  287 - 185 129  78
             272  287 - 136  87  44
             272  291 - 199 148  93
             280  309 -  89 124 149
             280  311 -  89 124 149
             291  321 -  89 124 149
             294  322 -  89 124 149
             294  322 -  89 124 149
             317  365 -  89 124 149
             318  365 -  89 124 149
             329  365 -  89 124 149
             340  365 -  89 124 149
             340  365 -  89 124 149
             343  373 -  89 124 149
             371  379 - 144  99  60
             371  379 - 144  99  60
             371  383 - 168 115  75
             374  388 -  60  38  25
             396  453 -  89 124 149
             397  453 -  89 124 149
             416  285 -  89 124 149
             205   26 -  76 165  90
              48   19 - 107 230  72
             244   58 - 217 156 101
             169  116 -  89 124 149
              59  138 - 181 134 104
              39  124 - 119  65  27
              39  124 - 119  65  27
              72  142 - 214 157 102
              93  150 - 131  75  52
             139  215 - 237 184 140
             145  225 - 171 109  68
             150  260 - 183 121  82
             151  265 - 167 104  61
             181  284 - 173 148 117
             187  291 - 185 133  86
             187  307 - 209 166 132
             191  318 - 222 175 133
             212  318 - 193 140 108
             220  326 - 136  94  72
             270  331 -  89 124 149
             272  331 -  89 124 149
        "#,
    )?;

    Ok(())
}
