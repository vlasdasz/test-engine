use contour::ContourBuilder;
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    OpenSimplex,
};
use test_engine::{
    gm::{
        flat::{IntSize, Point, PointsPath, Size},
        Color,
    },
    Image,
};
use ui::{layout::Anchor, refs::Weak, view, DrawMode, SubView, ViewData, ViewSetup, ViewTest, ViewTouch};
use ui_views::{AddLabel, DrawingView, ImageView, IntView};

#[view]
pub struct NoiseView {
    seed:           u32,
    image_view:     SubView<ImageView>,
    threshold_view: SubView<IntView>,
    x_view:         SubView<IntView>,
    y_view:         SubView<IntView>,
    size_view:      SubView<IntView>,
    drawing_view:   SubView<DrawingView>,
}

impl NoiseView {
    fn update_image(mut self: Weak<Self>) {
        let resolution: IntSize = (100, 100).into();
        let (image, contours) = generate_image(
            self.seed,
            resolution,
            (self.size_view.value(), self.size_view.value()).into(),
            (self.x_view.value(), self.y_view.value()).into(),
            self.threshold_view.value() as _,
        );

        self.image_view.image = image;

        self.drawing_view.remove_all_paths();
        for contour in contours {
            self.drawing_view.add_path(contour, &Color::TURQUOISE, DrawMode::Outline);
        }
    }
}

impl ViewSetup for NoiseView {
    fn setup(mut self: Weak<Self>) {
        self.image_view.place.back();

        self.enable_touch_low_priority();
        self.touch.up_inside.sub(move || self.update_image());

        let update_image = move |_| self.update_image();

        self.threshold_view
            .set_color(Color::WHITE)
            .set_value(124)
            .set_step(2)
            .add_label("there")
            .on_change(update_image)
            .place
            .size(40, 150)
            .bl(10);

        self.x_view
            .set_color(Color::WHITE)
            .set_value(65)
            .set_step(0.5)
            .add_label("x")
            .on_change(update_image);
        self.x_view
            .place
            .size(40, 150)
            .b(10)
            .anchor(Anchor::Left, self.threshold_view, 10);

        self.y_view
            .set_color(Color::WHITE)
            .set_value(8)
            .set_step(0.5)
            .add_label("y")
            .on_change(update_image);
        self.y_view.place.size(40, 150).b(10).anchor(Anchor::Left, self.x_view, 10);

        self.size_view
            .set_color(Color::WHITE)
            .set_value(6)
            .set_step(2)
            .add_label("size")
            .on_change(update_image);
        self.size_view.place.size(40, 150).b(10).anchor(Anchor::Left, self.y_view, 10);

        self.drawing_view.place.size(200, 200).br(0);
        self.drawing_view.set_color(Color::WHITE);

        self.update_image();
    }
}

impl ViewTest for NoiseView {
    fn test_size() -> IntSize
    where Self: Sized {
        (1200, 1000).into()
    }
}

fn extract_shapes(data: &[u8], resolution: IntSize) -> Vec<PointsPath> {
    let data: Vec<_> = data.iter().map(|val| *val as f32).collect();

    let c = ContourBuilder::new(resolution.width, resolution.height, false); // x dim., y dim., smoothing
    let res = c.contours(&data, &[0.5]).unwrap(); //

    res.first()
        .unwrap()
        .geometry()
        .into_iter()
        .map(|polygon| PointsPath {
            points: polygon
                .exterior()
                .into_iter()
                .map(|point| Point {
                    x: point.x as f32 + 10.0,
                    y: point.y as f32 + 10.0,
                })
                .into_iter()
                .step_by(5)
                .collect(),
        })
        .collect()
}

fn generate_image(
    seed: u32,
    resolution: IntSize,
    size: Size,
    position: Point,
    threshold: u8,
) -> (Weak<Image>, Vec<PointsPath>) {
    let open_simplex = OpenSimplex::new(seed);

    let half_w = size.width / 2.0;
    let half_h = size.width / 2.0;

    let map = PlaneMapBuilder::<_, 2>::new(&open_simplex)
        .set_size(resolution.width as usize, resolution.height as usize)
        .set_x_bounds((position.x - half_w) as f64, (position.x + half_w) as f64)
        .set_y_bounds((-position.y - half_h) as f64, (-position.y + half_h) as f64)
        .build();

    let (width, height) = map.size();
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

    for i in map {
        let val = ((i * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
        pixels.push(if val > threshold { 0 } else { 255 });
    }

    let contours = extract_shapes(&pixels, resolution);

    let image_name = format!("noise_image_{seed}_{resolution}_{size}_{position}_{threshold}");

    (
        Image::from(&pixels, (width, height).into(), 1, image_name),
        contours,
    )
}
