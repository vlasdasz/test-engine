use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    OpenSimplex,
};
use test_engine::{
    gm::{
        flat::{IntSize, Point, Size},
        Color,
    },
    Image,
};
use ui::{refs::Weak, view, SubView, ViewData, ViewSetup, ViewTest, ViewTouch};
use ui_views::{AddLabel, ImageView, IntView};

#[view]
pub struct NoiseView {
    seed:           u32,
    image_view:     SubView<ImageView>,
    threshold_view: SubView<IntView>,
}

impl NoiseView {
    fn update_image(mut self: Weak<Self>) {
        let resolution: IntSize = (500, 500).into();

        self.image_view.image = generate_image(
            self.seed,
            resolution,
            (20, 20).into(),
            (0, 0).into(),
            self.threshold_view.value() as _,
        );
    }
}

impl ViewSetup for NoiseView {
    fn setup(mut self: Weak<Self>) {
        self.image_view.place.back();

        self.enable_touch_low_priority();
        self.touch.up_inside.sub(move || self.update_image());

        self.threshold_view
            .set_color(Color::WHITE)
            .set_value(100)
            .set_step(2)
            .add_label("there")
            .place
            .size(40, 150)
            .bl(10);

        self.threshold_view.on_change(move |_| self.update_image());

        self.update_image();
    }
}

impl ViewTest for NoiseView {
    fn test_size() -> IntSize
    where Self: Sized {
        (1200, 1000).into()
    }
}

fn generate_image(seed: u32, resolution: IntSize, size: Size, position: Point, threshold: u8) -> Weak<Image> {
    let open_simplex = OpenSimplex::new(seed);

    let half_w = size.width / 2.0;
    let half_h = size.width / 2.0;

    let map = PlaneMapBuilder::<_, 2>::new(&open_simplex)
        .set_size(resolution.width as usize, resolution.height as usize)
        .set_x_bounds((position.x - half_w) as f64, (position.x + half_w) as f64)
        .set_y_bounds((position.y - half_h) as f64, (position.y + half_h) as f64)
        .build();

    let (width, height) = map.size();
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

    for i in map {
        let val = ((i * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
        pixels.push(if val > threshold { 255 } else { 0 });
    }

    let image_name = format!("noise_image_{seed}_{resolution}_{size}_{position}_{threshold}");

    Image::from(&pixels, (width, height).into(), 1, image_name)
}
