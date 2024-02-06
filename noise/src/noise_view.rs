use gen::noise::{generate_terrain, TerrainData, TerrainParams};
use test_engine::{
    gl_wrapper::path_data::DrawMode,
    gm::{
        flat::{IntSize, Points},
        Color,
    },
    GlImage,
};
use ui::{layout::Anchor, refs::Weak, view, SubView, ViewData, ViewSetup, ViewTest, ViewTouch};
use ui_views::{AddLabel, DrawingView, ImageView, IntView};

#[view]
pub struct NoiseView {
    seed:           u32,
    drawing_view:   SubView<DrawingView>,
    threshold_view: SubView<IntView>,
    x_view:         SubView<IntView>,
    y_view:         SubView<IntView>,
    size_view:      SubView<IntView>,
    image_view:     SubView<ImageView>,
}

impl NoiseView {
    fn update_image(mut self: Weak<Self>) {
        let resolution: IntSize = (100, 100).into();

        let (image, islands) = generate_image(TerrainParams {
            seed: self.seed,
            resolution,
            size: (self.size_view.value(), self.size_view.value()).into(),
            position: (self.x_view.value(), self.y_view.value()).into(),
            threshold: self.threshold_view.value() as _,
        });

        self.image_view.gl_image = image;

        self.drawing_view.remove_all_paths();
        for island in islands {
            self.drawing_view.add_path(island, &Color::TURQUOISE, DrawMode::Outline);
        }
    }
}

impl ViewSetup for NoiseView {
    fn setup(mut self: Weak<Self>) {
        self.drawing_view.place().back();
        self.drawing_view.set_color(Color::WHITE);

        self.enable_touch_low_priority();
        self.touch.up_inside.sub(move || self.update_image());

        let update_image = move |_| self.update_image();

        self.threshold_view
            .set_color(Color::WHITE)
            .set_value(124)
            .set_step(2)
            .add_label("there")
            .on_change(update_image)
            .place()
            .size(40, 150)
            .bl(10);

        self.x_view
            .set_color(Color::WHITE)
            .set_value(65)
            .set_step(0.5)
            .add_label("x")
            .on_change(update_image);
        self.x_view
            .place()
            .size(40, 150)
            .b(10)
            .anchor(Anchor::Left, self.threshold_view, 10);

        self.y_view
            .set_color(Color::WHITE)
            .set_value(8)
            .set_step(0.5)
            .add_label("y")
            .on_change(update_image);
        self.y_view.place().size(40, 150).b(10).anchor(Anchor::Left, self.x_view, 10);

        self.size_view
            .set_color(Color::WHITE)
            .set_value(6)
            .set_step(2)
            .add_label("size")
            .on_change(update_image);
        self.size_view.place().size(40, 150).b(10).anchor(Anchor::Left, self.y_view, 10);

        self.image_view.place().size(400, 400).br(0);

        self.update_image();
    }
}

impl ViewTest for NoiseView {
    fn test_size() -> IntSize
    where Self: Sized {
        (1200, 1000).into()
    }
}

fn generate_image(
    TerrainParams {
        seed,
        resolution,
        size,
        position,
        threshold,
    }: TerrainParams,
) -> (Weak<GlImage>, Vec<Points>) {
    let TerrainData { pixels, islands } = generate_terrain(TerrainParams {
        seed,
        resolution,
        size,
        position,
        threshold,
    });

    let image_name = format!("noise_image_{seed}_{resolution}_{size}_{position}_{threshold}");

    (
        GlImage::from(
            &pixels,
            (resolution.width, resolution.height).into(),
            1,
            image_name,
        ),
        islands,
    )
}
