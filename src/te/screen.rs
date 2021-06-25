use crate::gl_wrapper::gl_wrapper::Updatable;
use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Point, Rect, Size};
use crate::te::ui::DebugView;
use crate::te::{Assets, UIDrawer};
use crate::ui::input::touch::{ButtonState, Event, MouseButton};
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::ViewBase;
use tools::refs::make_box;
use tools::HasNew;

pub struct Screen {
    cursor_position: Point,
    root_view: Box<dyn View>,
    ui_drawer: UIDrawer,
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
        let ui_drawer = UIDrawer::new(assets);
        Screen {
            cursor_position: Point::new(),
            root_view: Box::new(ViewBase::new()),
            ui_drawer,
            char: 0,
        }
    }

    fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);
        self.root_view.add_subview(make_box(DebugView::new()));
    }

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.set_frame(Rect::from(size));
    }

    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    fn on_mouse_key_pressed(&mut self, _: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state),
        })
    }

    fn update(&mut self) {
        GLWrapper::clear();

        self.root_view
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw_view(&mut self.root_view);

        let font = &self.ui_drawer.assets.fonts.default;

        let image = &font.glyph_for_char(self.char as char).image;
        self.char += 1;
        if self.char > 120 {
            self.char = 0;
        }
        let mut rect = Rect::make(10.0, 10.0, 20.0, 20.0);
        rect.origin = self.ui_drawer.window_size.center();
        let color = Color::WHITE;

        self.ui_drawer.draw_image_in_rect(image, &rect, &color);
    }
}
