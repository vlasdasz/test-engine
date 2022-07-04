use gl_image::Image;
use gm::Color;
use rtools::{data_manager::Handle, Animation, Boxed, Rglica, ToRglica, Unwrap};

use crate::{
    basic::Button,
    complex::{DrawingView, StringCell, TableView, TableViewDataSource},
    data_source, impl_view, view,
    view::{ViewData, ViewFrame, ViewSubviews},
    ImageView, Label, View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct TestView {
    label:    Rglica<Label>,
    button:   Rglica<Button>,
    image:    Rglica<ImageView>,
    drawing:  Rglica<DrawingView>,
    table:    Rglica<TableView>,
    animated: Rglica<ImageView>,

    animation: Unwrap<Animation>,

    label_value: u64,
}

impl_view!(TestView);

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

impl ViewCallbacks for TestView {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.label.set_text("Hello label!");

        self.button = self.add_view();
        self.button.on_tap.set(self, |this, _| {
            let val = this.label_value;
            this.label.set_text(format!("Hello label! {}", val));
            this.label_value += 1;
        });

        self.image = self.add_view();

        self.drawing = self.add_view();
        self.drawing.add_path(
            vec![(20, 20), (30, 20), (20, 40), (30, 50), (1, 60), (1, 20)],
            Color::GREEN,
        );

        self.table = self.add_view();
        self.table.data_source = data_source!(self);
        self.table.reload_data();

        self.animated = self.add_view();
        self.animated.set_frame((100, 100));

        self.animation = Animation::new(0, 200, 10).into();
    }

    fn layout(&mut self) {
        self.deprecated_place().all_vertically();
        self.animated.set_y(self.animation.value());
    }
}

const DATA: &[&str; 3] = &["Solole", "Merkele", "Prokol"];

impl TableViewDataSource for TestView {
    fn number_of_cells(&self) -> usize {
        DATA.len()
    }

    fn cell_for_index(&self, index: usize) -> Box<dyn View> {
        let mut cell = StringCell::boxed();
        cell.set_data(DATA[index].into());
        cell
    }
}
