use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{ImageView, Setup, UI, ViewFrame, ViewSubviews, view},
    ui_test::check_colors,
};

#[view]
struct ImageFlip {
    #[init]
    tl: ImageView,
    tr: ImageView,
    bl: ImageView,
    br: ImageView,
}

impl Setup for ImageFlip {
    fn setup(mut self: Weak<Self>) {
        self.apply_to::<ImageView>(|i| {
            i.set_image("cat.png");
        });

        self.tl.set_frame((50, 50, 150, 150));
        self.tr.set_frame((250, 50, 150, 150)).flip_x = true;
        self.bl.set_frame((50, 250, 150, 150)).flip_y = true;
        self.br.set_frame((250, 250, 150, 150)).flip_x = true;
        self.br.flip_y = true;
    }
}

pub async fn test_image_flip() -> Result<()> {
    let _view = UI::init_test_view::<ImageFlip>().await;

    check_colors(
        r#"
              31   74 -  89 124 149
              79   74 - 230 190 191
             110  102 - 209 182 161
             128  102 - 200 174 149
             169  102 - 165 127 108
             180  102 - 158  99  81
             202  102 -  89 124 149
             289  102 - 186 158 136
             289  102 - 186 158 136
             372  102 - 235 208 199
             414  102 -  89 124 149
             435  192 -  89 124 149
             435  194 -  89 124 149
             374  267 - 237 203 194
             357  271 - 225 189 177
             353  277 - 219 184 164
             351  294 - 211 176 154
             332  325 - 132 101  73
             330  342 - 197 162 143
             330  345 - 173 143 119
             309  368 - 224 176 176
             302  393 - 224 182 183
             288  393 - 221 177 178
             232  422 -  89 124 149
             114  421 -  89 124 149
              61  421 -  89 124 149
              26  370 -  89 124 149
              29  329 -  89 124 149
              71  316 - 244 224 215
              97  315 - 227 190 174
             128  315 - 188 151 124
             158  310 - 202 166 142
             173  309 - 175 143 122
             188  304 - 168 140 118
             203  299 -  89 124 149
             260  281 - 154 126 104
             283  280 - 148 120  99
             292  280 - 169 136 117
             352  280 - 215 183 162
             412  280 -  89 124 149
             413  280 -  89 124 149
             336  425 -  89 124 149
             336  292 - 204 172 151
             282  423 -  89 124 149
             320  352 - 199 173 148
             320  329 - 168 138 112
             320  302 - 197 161 139
             320  272 - 206 171 151
             322  147 - 196 160 138
             324   99 - 210 184 159
             324   74 - 226 180 182
             324   52 - 224 184 185
             324   33 -  89 124 149
             121   29 -  89 124 149
             102   78 - 216 178 169
             111  100 - 214 185 167
             111  137 - 215 172 156
             113  153 - 208 173 151
             117  190 - 207 162 143
             117  229 -  89 124 149
             122  290 - 205 173 150
             122  318 - 188 147 115
             131  318 - 162 127  99
             139  318 - 178 143 115
             162  306 - 198 163 143
             212  293 -  89 124 149
             213  290 -  89 124 149
             144  303 - 205 169 147
             107  362 - 212 174 155
              92  362 - 212 164 144
              88  362 - 177 100  72
              95  305 - 221 186 167
             166  311 - 192 157 135
             158  323 - 184 150 122
             303  355 - 194 168 143
             363  357 - 233 210 204
             368  357 - 228 186 187
             368  337 - 239 212 201
             368  330 - 237 211 194
             342  294 - 213 181 160
             339  292 - 207 175 154
             246  278 -  89 124 149
             304  342 - 182 154 130
             311  361 - 195 168 147
             414  372 -  89 124 149
             338  134 - 176 124 100
             337  135 - 177 126 105
             308  140 - 187 141 115
             308  140 - 187 141 115
             303  137 - 193 146 116
             303  135 - 193 147 123
             303  135 - 193 147 123
             362  122 - 229 192 176
             364  122 - 232 198 186
             363   82 - 228 188 188
             362   81 - 228 188 188
             337   81 - 204 165 150
             277  105 - 142  91  72
             276  116 - 166 128 105
             372  122 - 240 213 202
             308  171 - 207 175 154
             171  172 - 153 125 103
             165  169 - 153 125 104
             119  169 - 206 171 149
             118  169 - 207 172 150
              79   86 - 228 188 189
             231   84 -  89 124 149
             123  119 - 191 160 132
             181  131 - 200 150 151
             194  131 - 200 148 150
             194  131 - 200 148 150
             194  127 - 199 149 150
             194  106 - 203 153 152
             170  103 - 146 101  82
              95  161 - 218 186 165
        "#,
    )
    .await?;

    Ok(())
}
