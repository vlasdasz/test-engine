use test_engine::{
    async_after, on_main,
    refs::Weak,
    ui::{
        link_button, view, Alert, Anchor, Button, Color, ColorMeter, Container, DPadView, DrawingView,
        ImageView, IntView, Label, PointsPath, PolygonMode, Spinner, StickView, Sub, TextField, ViewData,
        ViewSetup,
    },
    App,
};

#[view]
pub struct TestGameView {
    tl: Sub<Container>,
    tr: Sub<Container>,
    bl: Sub<Container>,
    br: Sub<Container>,

    drawing: Sub<DrawingView>,
    stick:   Sub<StickView>,

    image: Sub<ImageView>,

    label_l: Sub<Label>,
    image_r: Sub<ImageView>,

    dpad: Sub<DPadView>,
    int:  Sub<IntView>,

    spinner: Sub<Button>,
    alert:   Sub<Button>,

    color_meter: Sub<ColorMeter>,

    text_field: Sub<TextField>,

    objc: Sub<Button>,
}

impl ViewSetup for TestGameView {
    fn setup(mut self: Weak<Self>) {
        self.tl.set_color(Color::PURPLE).place().size(100, 100).tl(10);
        self.tr.set_color(Color::GREEN).place().size(100, 100).tr(10);
        self.bl.set_color(Color::BLUE).place().size(100, 100).bl(10);
        self.br.set_color(Color::ORANGE).place().size(100, 100).br(10);

        self.image.place().center().relative(Anchor::Size, self, 0.2);
        self.image.set_image("cat.png");

        self.label_l.place().center_y().relative(Anchor::Size, self, 0.2).anchor(
            Anchor::Right,
            self.image,
            20,
        );
        self.label_l.text = "Łėŵœ Ы".into();
        self.label_l.set_text_size(64.0);

        self.image_r.place().center_y().relative(Anchor::Size, self, 0.2).anchor(
            Anchor::Left,
            self.image,
            20,
        );
        self.image_r.set_image("palm.png");

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
        self.spinner.on_tap(|| {
            Spinner::start();
            async_after(4, async {
                Spinner::stop();
            });
        });

        self.alert.place().size(100, 28).anchor(Anchor::Left, self.int, 10).anchor(
            Anchor::Bot,
            self.spinner,
            10,
        );
        self.alert.set_text("Alert");
        self.alert.set_text_size(20);
        self.alert.on_tap(|| {
            Alert::show("Hello!");
            on_main(|| App::set_window_size((600, 600)))
        });

        self.color_meter.place().size(100, 100).b(10).anchor(Anchor::Right, self.br, 10);

        self.drawing.place().w(280).t(10).anchor(Anchor::Right, self.tr, 10).relative(
            Anchor::Height,
            self,
            0.2,
        );

        self.drawing.add_path(
            [(0, 0), (40, 20), (20, 200), (150, 20), (20, 50)],
            Color::GREEN,
            PolygonMode::Fill,
        );

        self.drawing.add_path(
            PointsPath::circle_triangles_with((200, 100), 50, 5),
            Color::TURQUOISE,
            PolygonMode::Fill,
        );

        self.stick.place().t(40).size(200, 200).anchor(Anchor::Right, self.drawing, 10);

        self.text_field.set_placeholder("Type here");
        self.text_field.place().size(200, 50).t(10).anchor(Anchor::Left, self.tl, 10);

        self.objc.set_text("objc");
        link_button!(self, objc, call_obj);
        self.objc
            .place()
            .size(100, 100)
            .t(200)
            .anchor(Anchor::Left, self.text_field, 10);

        unsafe {
            ios_init_text_field();
        }
    }
}

impl TestGameView {
    fn call_obj(self: Weak<Self>) {
        use objc::{
            class, msg_send,
            runtime::{Class, Object, Sel},
            sel, sel_impl,
        };

        dbg!("Helloy??");

        // UIWindow *currentWindow = [UIApplication sharedApplication].keyWindow;

        let ui_application_class: *const Class = class!(UIApplication);

        let shared_application: *mut Object = unsafe { msg_send![ui_application_class, sharedApplication] };

        let key_window_sel: Sel = sel!(keyWindow);

        let key_window: *mut Object =
            unsafe { msg_send![shared_application, performSelector: key_window_sel] };

        dbg!(&shared_application);
        dbg!(&key_window);

        dbg!("A");

        unsafe {
            ios_show_keyboard();
        }
    }
}

extern "C" {
    fn ios_init_text_field();
    fn ios_show_keyboard();
    fn ios_callback();
}
