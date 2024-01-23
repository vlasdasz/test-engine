use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    OpenSimplex,
};
use test_engine::{
    gm::flat::{IntSize, Point, Size},
    Image,
};
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest, ViewTouch};
use ui_views::{ImageView, IntView};

#[view]
pub struct NoiseView {
    seed:           u32,
    image_view:     SubView<ImageView>,
    threshold_view: SubView<IntView>,
}

impl NoiseView {}

impl ViewSetup for NoiseView {
    fn setup(mut self: Weak<Self>) {
        let resolution: IntSize = (100, 100).into();

        self.image_view.image = generate_image(self.seed, resolution, (20, 20).into(), (0, 0).into(), 100);
        self.image_view.place.back();

        self.enable_touch();
        self.touch.up_inside.sub(move || {
            self.image_view.image =
                generate_image(self.seed, resolution, (20, 20).into(), (0, 0).into(), 100);
        });

        // self.threshold_view.
    }
}

impl ViewTest for NoiseView {
    fn test_size() -> IntSize
    where Self: Sized {
        (1200, 1200).into()
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
