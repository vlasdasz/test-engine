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

    fn update_view(view: &mut Box<dyn View>) {
        view.update();
        for view in view.subviews_mut() {
            Screen::update_view(view);
        }
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
        let mut debug_view = DebugView::new();
        debug_view.font = self.ui_drawer.assets.fonts.default.clone();
        self.root_view.add_subview(make_box(debug_view));
        self.root_view
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
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

        Screen::update_view(&mut self.root_view);

        self.root_view
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw_view(&mut self.root_view);

        let font = &self.ui_drawer.assets.fonts.default;

        let image = &font.glyph_for_char(self.char as char).image;
        self.char += 1;
        if self.char > 120 {
            self.char = 0;
        }
        let mut rect = Rect::make(10, 10, 20, 20);
        rect.origin = self.ui_drawer.window_size.center();
        let color = Color::WHITE;

        self.ui_drawer.draw_image_in_rect(image, &rect, &color);
    }
}
