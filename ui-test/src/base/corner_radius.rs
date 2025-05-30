use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Anchor::Left, BLUE, Container, ImageView, Setup, UI, ViewData, YELLOW, view},
    ui_test::check_colors,
};

#[view]
struct CornerRadius {
    #[init]
    square: Container,
    image:  ImageView,
    wide:   Container,
}

impl Setup for CornerRadius {
    fn setup(mut self: Weak<Self>) {
        self.square.set_color(BLUE).set_corner_radius(50);
        self.square.place().size(100, 100).tl(50);

        self.image.set_image("cat.png").set_corner_radius(40);
        self.image.place().size(100, 200).t(50).anchor(Left, self.square, 20);

        self.wide.set_color(YELLOW).set_corner_radius(20);
        self.wide.place().size(200, 100).t(50).anchor(Left, self.image, 20);
    }
}

pub async fn test_corner_radius() -> Result<()> {
    UI::init_test_view::<CornerRadius>().await;

    check_colors(
        r#"
              40   56 -  89 124 149
              54   55 -  89 124 149
              65   57 -  89 124 149
              87   59 -   0   0 231
             105   63 -   0   0 231
             118   59 -   0   0 231
             129   55 -  89 124 149
             138   55 -  89 124 149
             141   55 -  89 124 149
             142   65 -  89 124 149
             142   88 -   0   0 231
             126   98 -   0   0 231
             126  109 -   0   0 231
             134  136 -  89 124 149
             136  150 -  89 124 149
             122  143 -   0   0 231
             105  123 -   0   0 231
             100  123 -   0   0 231
              91  146 -   0   0 231
              85  160 -  89 124 149
              81  143 -   0   0 231
              80  127 -   0   0 231
              57  123 -   0   0 231
              38  131 -  89 124 149
              50  121 -  89 124 149
              69  105 -   0   0 231
              72   86 -   0   0 231
              55   70 -  89 124 149
              47   46 -  89 124 149
              78   60 -   0   0 231
              95  103 -   0   0 231
        "#,
    )
    .await?;

    check_colors(
        r#"
             172  241 -  89 124 149
             174  239 -  89 124 149
             189  221 - 236 199 191
             202  223 - 216 178 165
             216  239 - 213 168 149
             233  251 -  89 124 149
             242  241 - 152 122  98
             238  228 - 173 141 120
             245  227 - 133 104  86
             256  240 -  89 124 149
             268  251 -  89 124 149
             262  225 - 167 137 113
             260  200 - 156 128 106
             274  184 -  89 124 149
             286  182 -  89 124 149
             258  175 - 163 135 113
             261  150 - 201 151 152
             275  119 -  89 124 149
             264  108 - 203 154 149
             242   81 - 219 173 173
             266   37 -  89 124 149
             270   35 -  89 124 149
             257   67 - 214 164 163
             248   70 - 220 174 176
             224   42 -  89 124 149
             223   38 -  89 124 149
             214   86 - 220 183 175
             181   54 -  89 124 149
             172   47 -  89 124 149
             167   47 -  89 124 149
             187   74 - 230 191 192
             171   68 -  89 124 149
             152   61 -  89 124 149
             158   77 -  89 124 149
             185  143 - 241 214 205
             167  168 -  89 124 149
             153  187 -  89 124 149
             178  196 - 232 188 189
             172  247 -  89 124 149
             190  228 - 234 197 189
        "#,
    )
    .await?;

    check_colors(
        r#"
             294  152 -  89 124 149
             294  140 - 255 255   0
             299  130 - 255 255   0
             304  126 - 255 255   0
             294  142 - 255 255   0
             288  151 -  89 124 149
             295  126 - 255 255   0
             296  101 - 255 255   0
             291   73 - 255 255   0
             288   16 -  89 124 149
             286   29 -  89 124 149
             296   46 -  89 124 149
             299   64 - 255 255   0
             285   45 -  89 124 149
             300   61 - 255 255   0
             304   62 - 255 255   0
             307   43 -  89 124 149
             377   36 -  89 124 149
             423   51 - 255 255   0
             497   52 -  89 124 149
             516   50 -  89 124 149
             484   61 - 255 255   0
             480   71 - 255 255   0
             491   48 -  89 124 149
             494   42 -  89 124 149
             468   65 - 255 255   0
             483   51 -  89 124 149
             490   42 -  89 124 149
             481   99 - 255 255   0
             467  129 - 255 255   0
             473  136 - 255 255   0
             480  142 - 255 255   0
             496  155 -  89 124 149
             492  156 -  89 124 149
             469  137 - 255 255   0
             463  143 - 255 255   0
             415  181 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
