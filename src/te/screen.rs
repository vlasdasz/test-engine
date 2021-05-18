use crate::gl_wrapper::gl_wrapper::Updatable;
use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Point, Rect, Size};
use crate::te::{Assets, TEUIDrawer};
use crate::tools::weak_self::HasWeakSelf;
use crate::ui::input::touch::{ButtonState, Event, MouseButton};
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::{ImageView, Label, ViewBase};
use tools::refs::{make_shared, Shared};
use tools::New;

pub struct Screen {
    cursor_position: Point,
    root_view: Shared<ViewBase>,
    ui_drawer: TEUIDrawer,
    char: u8,
}

impl Screen {
    fn on_touch(&mut self, mut touch: Touch) {
        self.root_view
            .try_borrow_mut()
            .unwrap()
            .check_touch(&mut touch)
    }
}

impl Updatable for Screen {
    fn new() -> Screen {
        let assets = Assets::init();
        let ui_drawer = TEUIDrawer::new(assets);
        Screen {
            cursor_position: Point::new(),
            root_view: ViewBase::new_shared(),
            ui_drawer,
            char: 0,
        }
    }

    fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        let mut label = Label::new();
        label.font = self.ui_drawer.assets.fonts.default.clone();
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadag4ka");

        self.root_view.borrow_mut().add_subview(make_shared(label));

        let mut image_view = ImageView::new();

        image_view.image = self.ui_drawer.assets.images.cat;
        image_view.set_frame(Rect::make(10.0, 100.0, 200.0, 200.0));
        self.root_view
            .borrow_mut()
            .add_subview(make_shared(image_view));

        self.root_view.borrow_mut().make_subview(|view| {
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
        self.root_view
            .borrow_mut()
            .set_frame(Rect::from_size(&size));
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

        self.ui_drawer.draw_view(self.root_view.clone());

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
