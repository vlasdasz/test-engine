use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Point, Rect, Size};
use crate::sprites::Control;
use crate::sprites::Level;
use crate::te::paths;
use crate::te::sprites::sprites_drawer::SpritesDrawer;
use crate::te::ui::{DebugView, TestView};
use crate::te::{Assets, UIDrawer};
use crate::ui::input::touch::{ButtonState, Event, MouseButton};
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::Font;
use crate::ui::ViewBase;
use lazy_static::lazy_static;
use std::rc::Rc;
use std::sync::Mutex;
use tools::has_new::new;
use tools::refs::{make_shared, new_shared, Shared};
use tools::New;

lazy_static! {
    pub static ref DEFAULT_FONT: Mutex<Font> =
        Mutex::new(Font::new(&paths::fonts().join("SF.otf"), 48).unwrap());
}

pub struct Screen {
    cursor_position: Point,
    assets: Rc<Assets>,
    debug_view: Shared<DebugView>,
    root_view: Shared<dyn View>,
    level: Shared<Level>,
    ui_drawer: UIDrawer,
    sprites_drawer: SpritesDrawer,
}

impl Screen {
    pub fn on_touch(&self, mut touch: Touch) {
        self.root_view.borrow().check_touch(&mut touch);
        self.debug_view.borrow().check_touch(&mut touch);
    }

    fn update_view(view: Shared<dyn View>) {
        let mut view = view.try_borrow_mut().unwrap();
        view.update();
        for view in view.subviews_mut() {
            Screen::update_view(view.clone());
        }
    }

    pub fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        self.root_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());

        self.debug_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());

        self.debug_view.borrow_mut().setup(self.debug_view.clone());

        self.setup_level();
        self.setup_test_view();
    }

    fn setup_level(&mut self) {
        let mut level = self.level.borrow_mut();

        level.setup();

        level.add_collider(new(), Size::make(100, 1));

        level.add_collider(Point::make(20, 0), Size::make(1, 100));
        level.add_collider(Point::make(-20, 0), Size::make(1, 100));

        for i in 0..500 {
            level.add_rect(Point::make(0.1 * i as f32, i * 2), Size::square(0.5));
        }
    }

    fn setup_test_view(&mut self) {
        let view = TestView::new();

        let a = self.level.clone();
        view.dpad.borrow_mut().on_up.subscribe(move |_| {
            a.borrow_mut().jump();
        });

        let a = self.level.clone();
        view.dpad.borrow_mut().on_left.subscribe(move |_| {
            a.borrow_mut().go_left();
        });

        let a = self.level.clone();
        view.dpad.borrow_mut().on_right.subscribe(move |_| {
            a.borrow_mut().go_right();
        });

        self.root_view.borrow_mut().add_subview(make_shared(view));
    }

    pub fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.borrow_mut().set_frame(Rect::from(size));
        self.sprites_drawer.set_resolution(&size);
        self.sprites_drawer.set_camera_position(&Point::make(0, 0));
        self.update();
    }

    pub fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    pub fn on_mouse_key_pressed(&self, _: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state),
        })
    }

    pub fn update(&mut self) {
        GLWrapper::clear();

        let mut level = self.level.borrow_mut();

        level.update();

        self.sprites_drawer
            .set_camera_position(&level.player.borrow().position);

        for sprite in &level.sprites {
            let sprite = sprite.borrow();
            self.sprites_drawer.draw(&sprite);
        }

        for wall in &level.walls {
            let wall = wall.borrow();
            self.sprites_drawer.draw(&wall);
        }

        Screen::update_view(self.root_view.clone());
        Screen::update_view(self.debug_view.clone());

        self.root_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw(self.root_view.clone());

        self.debug_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw(self.debug_view.clone());

        self.ui_drawer.reset_viewport();
    }
}

impl New for Screen {
    fn new() -> Screen {
        let assets = Assets::init();
        Screen {
            cursor_position: Point::new(),
            assets: assets.clone(),
            debug_view: new_shared::<DebugView>(),
            root_view: new_shared::<ViewBase>(),
            level: new_shared(),
            ui_drawer: UIDrawer::new(assets.clone()),
            sprites_drawer: SpritesDrawer::new(assets),
        }
    }
}
