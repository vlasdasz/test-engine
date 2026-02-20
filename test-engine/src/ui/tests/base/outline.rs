use anyhow::Result;
use gm::color::{BLUE, GREEN, RED, YELLOW};
use hreads::from_main;
use refs::Weak;
use ui::{Anchor::Left, Container, ImageView, Setup, ViewData, ViewTest, view_test};

use crate as test_engine;
use crate::ui_test::check_colors;

#[view_test]
struct Outline {
    #[init]
    square: Container,
    image:  ImageView,
    wide:   Container,
}

impl Setup for Outline {
    fn setup(self: Weak<Self>) {
        self.square.set_color(BLUE).set_border_width(10).set_border_color(RED);
        self.square.place().size(100, 100).tl(50);

        self.image.set_image("cat.png").set_border_width(5).set_border_color(GREEN);
        self.image.place().size(100, 200).t(50).anchor(Left, self.square, 20);

        self.wide.set_color(YELLOW).set_border_width(25).set_border_color(BLUE);
        self.wide.place().size(200, 100).t(50).anchor(Left, self.image, 20);
    }
}

impl ViewTest for Outline {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
                  46  119 -  89 124 149
                  57  120 - 255   0   0
                  84  123 -   0   0 231
                 134  121 -   0   0 231
                 146  121 - 255   0   0
                 155  123 -  89 124 149
                 143  157 -  89 124 149
                 148  150 -  89 124 149
                 153  147 -  89 124 149
                 144   54 - 255   0   0
                 124   40 -  89 124 149
                 164   68 -  89 124 149
                 173   66 -   0 255   0
                 180   65 - 231 192 197
                 190   51 -   0 255   0
                 204   39 -  89 124 149
                 259   42 -  89 124 149
                 267   54 -   0 255   0
                 272   71 -  89 124 149
                 244   62 - 223 177 179
                 269  100 -   0 255   0
                 241  267 -  89 124 149
                 250  250 -  89 124 149
                 285  155 -  89 124 149
                 298  146 -   0   0 231
                 306  135 -   0   0 231
                 319  122 - 255 255   0
                 285   41 -  89 124 149
                 297   58 -   0   0 231
                 309   66 -   0   0 231
                 321   82 - 255 255   0
                 453   81 - 255 255   0
                 463   63 -   0   0 231
                 483   45 -  89 124 149
                 488   59 -   0   0 231
                 485  145 -   0   0 231
            ",
        )?;

        from_main(move || {
            view.square.set_corner_radius(15);
            view.image.set_corner_radius(25);
            view.wide.set_corner_radius(40);
        });

        check_colors(
            r"
          46   65 -  89 124 149
          56   58 - 255   0   0
          64   42 -  89 124 149
          46   50 -  89 124 149
          62   63 -   0   0 231
          67   71 -   0   0 231
          52  152 -  89 124 149
          56  146 - 255   0   0
          59  141 - 255   0   0
          65  136 -   0   0 231
         132  134 -   0   0 231
         136  138 -   0   0 231
         141  143 - 255   0   0
         142  143 - 255   0   0
         145  149 -  89 124 149
         172  246 -  89 124 149
         178  242 -   0 255   0
         184  238 - 227 188 181
         166   53 -  89 124 149
         171   56 -  89 124 149
         177   68 - 233 194 199
         180   95 - 228 188 189
         171  118 -   0 255   0
         160  125 -  89 124 149
         305  147 -  89 124 149
         322  128 -   0   0 231
         325  117 - 255 255   0
         316  116 - 255 255   0
         315  131 -   0   0 231
         314  155 -  89 124 149
         341  143 -   0   0 231
         291   54 -  89 124 149
         301   62 -   0   0 231
         312   68 -   0   0 231
         331   87 - 255 255   0
    ",
        )?;

        // crate::ui_test::record_ui_test();

        Ok(())
    }
}
