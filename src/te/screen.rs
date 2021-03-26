use crate::gm::{Size, Rect, Color};
use crate::te::{TEUIDrawer, Assets};
use crate::ui::View;
use crate::gl_wrapper::Updatable;

pub struct Screen {
    root_view: View,
    window_size: Size,
    ui_drawer: TEUIDrawer
}

impl Screen {
}

impl Updatable for Screen {
    fn new() -> Screen {
        let assets = Assets::init();
        let ui_drawer = TEUIDrawer::new(assets);
        Screen { root_view: View::new(), ui_drawer, window_size: Size::new() }
    }
    fn init(&mut self) {
        self.root_view.make_subview(|view|{

            view.set_frame(Rect::make(200.0, 200.0, 300.0, 300.0));
            view.color = Color::BLUE;

            view.make_subview(|view|{
                view.set_frame(Rect::make(20.0, 20.0, 100.0, 100.0));
                view.color = Color::GREEN;

                view.make_subview(|view| {
                    view.set_frame(Rect::make(10.0, 10.0, 20.0, 20.0));
                    view.color = Color::TURQUOISE;
                });

            });

        });
    }
    fn update(&mut self, windows_size: &Size) {
        self.window_size = *windows_size;
        self.root_view.set_frame(Rect::from_size(windows_size));
        self.ui_drawer.set_size(windows_size);
        self.ui_drawer.draw_view(&mut self.root_view);
    }
}
