use test_engine::{
    async_after, on_main,
    refs::Weak,
    ui::{
        async_link_button, view, Alert, Anchor, Button, Color, Container, DPadView, Image, ImageView,
        IntView, Label, Spinner, SubView, ViewData, ViewSetup,
    },
    App, DataManager,
};

#[view]
pub struct TestGameView {
    tl: SubView<Container>,
    tr: SubView<Container>,
    bl: SubView<Container>,
    br: SubView<Container>,

    image: SubView<ImageView>,

    label_l: SubView<Label>,
    label_r: SubView<Label>,

    dpad: SubView<DPadView>,
    int:  SubView<IntView>,

    spinner: SubView<Button>,
    alert:   SubView<Button>,
}

impl TestGameView {
    async fn spinner_pressed(self: Weak<Self>) {
        Spinner::start();
        async_after(4, async {
            Spinner::stop();
        });
    }

    async fn alert_pressed(self: Weak<Self>) {
        Alert::show("Hello!");
        on_main(|| App::set_window_size((600, 600)))
    }
}

impl ViewSetup for TestGameView {
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
        self.label_l.set_text_size(64.0);

        self.label_r.place().center_y().relative(Anchor::Size, self, 0.2).anchor(
            Anchor::Left,
            self.image,
            20,
        );
        self.label_r.text = "щКыЩъ".into();

        self.dpad.place().size(200, 140).b(20).anchor(Anchor::Left, self.bl, 10);

        self.dpad.on_press.val(move |direction| {
            self.label_l.set_text(format!("{direction:?}"));
            App::set_window_title(format!("{direction:?}"));
        });

        self.int.place().size(80, 150).b(20).anchor(Anchor::Left, self.dpad, 10);

        self.spinner.place().size(100, 28).b(20).anchor(Anchor::Left, self.int, 10);
        self.spinner.set_text("Spinner");
        self.spinner.set_text_size(20);
        async_link_button!(self, spinner, spinner_pressed);

        self.alert.place().size(100, 28).anchor(Anchor::Left, self.int, 10).anchor(
            Anchor::Bot,
            self.spinner,
            10,
        );
        self.alert.set_text("Alert");
        self.alert.set_text_size(20);
        async_link_button!(self, alert, alert_pressed);
    }
}
