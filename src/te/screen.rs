use crate::gl_wrapper::gl_wrapper::Updatable;
use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Point, Rect, Size};
use crate::te::{Assets, TEUIDrawer};
use crate::ui::input::touch::{ButtonState, Event, MouseButton};
use crate::ui::input::Touch;

use crate::ui::{ImageView, Label, ViewBase};

use crate::ui::view::View;
use tools::refs::make_shared;
use tools::HasNew;

pub struct Screen {
    cursor_position: Point,
    root_view: ViewBase,
    ui_drawer: TEUIDrawer,
    char: u8,
}

impl Screen {
    fn on_touch(&mut self, mut touch: Touch) {
        self.root_view.check_touch(&mut touch)
    }
}

impl Updatable for Screen {
    fn new() -> Screen {
        let assets = Assets::init();
        let ui_drawer = TEUIDrawer::new(assets);
        Screen {
            cursor_position: Point::new(),
            root_view: ViewBase::new(),
            ui_drawer,
            char: 0,
        }
    }

    fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        let mut label = Label::from_rect(Rect::make(500.0, 100.0, 200.0, 200.0));
        label.font = self.ui_drawer.assets.fonts.default.clone();
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadag4ka");

        self.root_view.add_subview(make_shared(label));

        let mut image_view = ImageView::from_rect(Rect::make(100.0, 100.0, 200.0, 200.0));
        image_view.image = self.ui_drawer.assets.images.cat;

        let mut image2 = ImageView::from_rect(Rect::make(10.0, 10.0, 100.0, 100.0));
        image2.image = self.ui_drawer.assets.images.cat;

        let mut image3 = ImageView::from_rect(Rect::make(10.0, 10.0, 50.0, 50.0));
        image3.image = self.ui_drawer.assets.images.cat;
        image2.add_subview(make_shared(image3));

        image_view.add_subview(make_shared(image2));

        self.root_view.add_subview(make_shared(image_view));

        self.root_view.make_subview(|view| {
            view.set_frame(Rect::make(200.0, 200.0, 300.0, 300.0));
            view.set_color(Color::BLUE);

            view.make_subview(|view| {
                view.set_frame(Rect::make(20.0, 20.0, 100.0, 100.0));
                view.set_color(Color::GREEN);

                view.make_subview(|view| {
                    view.set_frame(Rect::make(10.0, 10.0, 20.0, 20.0));
                    view.enable_touch();
                    view.set_color(Color::TURQUOISE);
                });
            });
        });
    }

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.set_frame(Rect::from(size));
    }

    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    fn on_mouse_key_pressed(&mut self, button: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state),
        })
    }

    fn update(&mut self) {
        GLWrapper::clear();

        self.ui_drawer.draw_view(&mut self.root_view);

        let font = &self.ui_drawer.assets.fonts.default;

        let image = &font.glyph_for_char(self.char as char).image;
        self.char += 1;
        if self.char > 120 {
            self.char = 0;
        }
        let rect = Rect::make(10.0, 10.0, 20.0, 20.0);
        let color = Color::WHITE;

        self.ui_drawer.draw_image_in_rect(image, &rect, &color);
    }
}
