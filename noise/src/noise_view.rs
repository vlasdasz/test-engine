use gen::noise::{generate_terrain, TerrainData, TerrainParams};
use test_engine::{
    gm::LossyConvert,
    refs::{Own, Weak},
    ui::{
        view, AddLabel, Anchor, Button, Color, DebugView, DrawingView, Image, ImageView, NumberView, Point,
        PointsPath, Size, ViewData, ViewSetup, ViewTouch,
    },
    Event,
};

#[view]
pub struct NoiseView {
    seed: u32,

    on_back: Event,

    #[init]
    drawing_view:   DrawingView,
    threshold_view: NumberView<f32>,
    x_view:         NumberView<f32>,
    y_view:         NumberView<f32>,
    size_view:      NumberView<f32>,
    image_view:     ImageView,
    back:           Button,
}

impl NoiseView {
    fn update_image(mut self: Weak<Self>) {
        let resolution: Size<u32> = (100, 100).into();

        let (image, islands) = generate_image(TerrainParams {
            seed: self.seed,
            resolution,
            size: (self.size_view.value(), self.size_view.value()).into(),
            position: (self.x_view.value(), self.y_view.value()).into(),
            threshold: self.threshold_view.value().lossy_convert(),
            skip: 3,
        });

        self.image_view.set_image(image);

        self.drawing_view.remove_all_paths();

        for island in islands {
            self.drawing_view.add_path(island.into_iter().map(|a| a * 20), Color::BLACK);
        }

        self.drawing_view.add_path(
            PointsPath::circle_triangles_with((200, 100), 50, 5),
            Color::TURQUOISE,
        );
    }

    pub fn on_back(self: Own<Self>, callback: impl FnMut() + 'static) -> Own<Self> {
        self.on_back.sub(callback);
        self
    }
}

impl ViewSetup for NoiseView {
    fn setup(mut self: Weak<Self>) {
        DebugView::disable();

        self.drawing_view.place().back();
        self.drawing_view.set_color(Color::WHITE);

        self.enable_touch_low_priority();
        self.touch.up_inside.sub(move || self.update_image());

        let update_image = move |_| self.update_image();

        self.threshold_view
            .set_color(Color::WHITE)
            .set_value(124.0)
            .set_step(2.0)
            .add_label("there")
            .on_change(update_image)
            .place()
            .size(40, 150)
            .bl(10);

        self.x_view
            .set_color(Color::WHITE)
            .set_value(65.0)
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
            .set_value(8.0)
            .set_step(0.5)
            .add_label("y")
            .on_change(update_image);
        self.y_view.place().size(40, 150).b(10).anchor(Anchor::Left, self.x_view, 10);

        self.size_view
            .set_color(Color::WHITE)
            .set_value(6.0)
            .set_step(2.0)
            .add_label("size")
            .on_change(update_image);
        self.size_view.place().size(40, 150).b(10).anchor(Anchor::Left, self.y_view, 10);

        self.image_view.place().size(400, 400).br(0);

        self.back.set_text("Back");
        self.back.place().t(200).l(10).size(100, 50);
        self.back.on_tap(move || {
            self.on_back.trigger(());
        });

        self.update_image();
    }
}

fn generate_image(
    TerrainParams {
        seed,
        resolution,
        size,
        position,
        threshold,
        skip,
    }: TerrainParams,
) -> (Weak<Image>, Vec<Vec<Point>>) {
    let TerrainData { pixels, islands } = generate_terrain(TerrainParams {
        seed,
        resolution,
        size,
        position,
        threshold,
        skip,
    });

    let image_name = format!("noise_image_{seed}_{resolution}_{size}_{position}_{threshold}");

    (
        Image::from_raw_data(
            &pixels,
            image_name,
            (resolution.width, resolution.height).into(),
            1,
        )
        .unwrap(),
        islands,
    )
}
