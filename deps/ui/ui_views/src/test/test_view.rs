use gl_image::Image;
use gm::{flat::PointsPath, Color};
use refs::{Own, Weak};
use rtools::{data_manager::Handle, Animation, Unwrap};
use ui::{view, DrawMode, SubView, View, ViewCallbacks, ViewData, ViewFrame, ViewSetup};

use crate::{data_source, Button, DrawingView, ImageView, Label, StringCell, TableView, TableViewDataSource};

#[view]
#[derive(Default)]
pub struct TestView {
    label:    SubView<Label>,
    button:   SubView<Button>,
    image:    SubView<ImageView>,
    drawing:  SubView<DrawingView>,
    table:    SubView<TableView>,
    animated: SubView<ImageView>,

    animation: Unwrap<Animation>,

    label_value: u64,
}

impl TestView {
    pub fn set_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.image.set_image(image);
        self
    }

    pub fn set_button_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.button.set_image(image);
        self
    }

    pub fn set_animation_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.animated.set_image(image);
        self
    }
}

impl ViewSetup for TestView {
    fn setup(mut self: Weak<Self>) {
        self.place.all_ver();

        self.label.set_text("Hello label!");

        self.button.on_tap.sub(move |_| {
            let val = self.label_value;
            self.label.set_text(format!("Hello label! {val}"));
            self.label_value += 1;
        });

        self.drawing.add_path(
            PointsPath::rounded_rect((0, 0, 100, 40), 15, 50),
            &Color::GREEN,
            DrawMode::Outline,
        );

        self.table.data_source = data_source!(self);
        self.table.reload_data();

        self.animated.set_frame((100, 100));

        self.animation = Animation::new(0, 200, 10).into();
    }
}

impl ViewCallbacks for TestView {
    fn update(&mut self) {
        self.animated.set_y(self.animation.value());
        let radius = self.button.frame().size.height / 2.0;
        self.button.set_corner_radius(radius);
        self.button.set_size((radius * 2.0, radius * 2.0));
    }
}

const DATA: &[&str; 3] = &["Solole", "Merkele", "Prokol"];

impl TableViewDataSource for TestView {
    fn number_of_cells(&self) -> usize {
        DATA.len()
    }

    fn cell_for_index(&self, index: usize) -> Own<dyn View> {
        let mut cell = Own::<StringCell>::default();
        cell.set_data(DATA[index].into());
        cell
    }

    fn cell_selected(&mut self, index: usize) {
        dbg!(index);
    }
}
