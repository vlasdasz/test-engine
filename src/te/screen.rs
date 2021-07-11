use crate::gl_wrapper::gl_wrapper::Updatable;
use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Point, Rect, Size};
use crate::te::paths;
use crate::te::ui::{DebugView, TestView};
use crate::te::{Assets, UIDrawer};
use crate::ui::input::touch::{ButtonState, Event, MouseButton};
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::Font;
use crate::ui::ViewBase;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tools::refs::{make_shared, Shared};
use tools::HasNew;

lazy_static! {
    pub static ref DEFAULT_FONT: Mutex<Font> =
        Mutex::new(Font::new(&paths::fonts().join("SF.otf"), 48).unwrap());
}

pub struct Screen {
    cursor_position: Point,
    debug_view: Shared<DebugView>,
    root_view: Shared<dyn View>,
    ui_drawer: UIDrawer,
}

impl Screen {
    pub fn on_touch(&self, mut touch: Touch) {
        self.root_view.borrow().check_touch(&mut touch);
    }

    fn update_view(view: Shared<dyn View>) {
        let mut view = view.try_borrow_mut().unwrap();
        view.update();
        for view in view.subviews_mut() {
            Screen::update_view(view.clone());
        }
    }
}

impl Updatable for Screen {
    fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        self.root_view
            .borrow_mut()
            .add_subview(make_shared(TestView::new()));
        self.root_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
    }

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.borrow_mut().set_frame(Rect::from(size));
        self.update();
    }

    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    fn on_mouse_key_pressed(&self, _: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state),
        })
    }

    fn update(&mut self) {
        GLWrapper::clear();

        Screen::update_view(self.root_view.clone());

        self.root_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw_view(self.root_view.clone());

        self.debug_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw_view(self.debug_view.clone());
    }
}

impl HasNew for Screen {
    fn new() -> Screen {
        let assets = Assets::init();
        Screen {
            cursor_position: Point::new(),
            debug_view: make_shared(DebugView::new()),
            root_view: make_shared(ViewBase::new()),
            ui_drawer: UIDrawer::new(assets),
        }
    }
}
