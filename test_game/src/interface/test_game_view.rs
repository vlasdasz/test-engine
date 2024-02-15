use std::mem::size_of;

use test_engine::{
    async_after, cast_slice, on_main,
    refs::Weak,
    ui::{
        async_link_button, view, Alert, Anchor, Button, Color, Container, DPadView, Image, ImageView,
        IntView, Label, Spinner, SubView, Touch, U8Color, View, ViewCallbacks, ViewData, ViewSetup,
        ViewTouch,
    },
    App, DataManager,
};
use tokio::spawn;

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

    fn on_touch(mut self: Weak<Self>, touch: Touch) {
        spawn(async move {
            let (buffer, width_bytes) = App::request_read_display().await.unwrap();
            let width_colors = width_bytes / size_of::<U8Color>() as u64;

            let data: &[u8] = &buffer.slice(..).get_mapped_range();
            let data: &[U8Color] = cast_slice(data);
            let color = data[(width_colors as f32 * touch.position.y) as usize + touch.position.x as usize];
            dbg!(&color);
            on_main(move || {
                self.set_color(color);
            });
        });
    }
}

impl ViewCallbacks for TestGameView {
    fn update(&mut self) {
        let cursor_pos = App::current().cursor_position;

        let mut this = self.weak_view();
        spawn(async move {
            let Ok((buffer, width_bytes)) = App::request_read_display().await else {
                return;
            };
            let width_colors = width_bytes / size_of::<U8Color>() as u64;

            let data: &[u8] = &buffer.slice(..).get_mapped_range();
            let data: &[U8Color] = cast_slice(data);
            let color = data[(width_colors as f32 * cursor_pos.y) as usize + cursor_pos.x as usize];
            //dbg!(&color);
            on_main(move || {
                this.set_color(color);
            });
        });
    }
}

impl ViewSetup for TestGameView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::LIGHTER_GRAY);
        self.enable_touch();

        self.touch.began.val(move |touch| {
            self.on_touch(touch);
        });

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

            if direction.is_up() {
                App::set_window_title(format!("{direction:?} read pixel"));
            }
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
