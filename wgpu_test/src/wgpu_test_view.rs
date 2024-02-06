use test_engine::{
    gm::Color,
    manage::data_manager::DataManager,
    ui,
    ui::{layout::Anchor, refs::Weak, Container, SubView, ViewData, ViewSetup},
    ui_views::{ImageView, Label},
    view,
    wgpu_wrapper::image::Image,
};

#[view]
pub struct WGPUTestView {
    tl: SubView<Container>,
    tr: SubView<Container>,
    bl: SubView<Container>,
    br: SubView<Container>,

    image: SubView<ImageView>,

    label_l: SubView<Label>,
    label_r: SubView<Label>,
}

impl ViewSetup for WGPUTestView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::LIGHTER_GRAY);

        self.tl.set_color(Color::RED).place().size(100, 100).tl(10);
        self.tr.set_color(Color::GREEN).place().size(100, 100).tr(10);
        self.bl.set_color(Color::BLUE).place().size(100, 100).bl(10);
        self.br.set_color(Color::ORANGE).place().size(100, 100).br(10);

        self.image.place().center().relative(Anchor::Size, self, 0.2);
        self.image.image = Image::get("cat.png");

        self.label_l.place().center_y().relative(Anchor::Size, self, 0.2).anchor(
            Anchor::Right,
            self.image,
            20,
        );
        self.label_l.text = "Łėŵœ Ы".into();

        self.label_r.place().center_y().relative(Anchor::Size, self, 0.2).anchor(
            Anchor::Left,
            self.image,
            20,
        );
        self.label_r.text = "щКыЩъ".into();
    }
}
