use std::{ops::DerefMut, rc::Rc};

use gl_image::Image;
use gl_wrapper::{DesktopInput, GLWrapper, Screen};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Action, Key};
use gm::{Color, Point, Rect, Size};
use sprites::{Control, Level, LevelBase, Sprite};
use tools::{Boxed, New, ToRglica};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use ui::input::touch::{ButtonState, Event};
use ui::{input::Touch, make_view_on, View, ViewBase};

use crate::{
    assets::Assets,
    paths,
    sprites::SpritesDrawer,
    ui::{ui_drawer::UIDrawer, DebugView, TestView},
};

pub struct TestScreen {
    cursor_position: Point,
    assets:          Rc<Assets>,
    root_view:       Box<dyn View>,
    level:           Box<dyn Level>,
    ui_drawer:       UIDrawer,
    sprites_drawer:  SpritesDrawer,
}

impl TestScreen {
    pub fn on_touch(&mut self, mut touch: Touch) { self.root_view.check_touch(&mut touch); }

    fn update_view(view: &mut Box<dyn View>) {
        view.update();
        for view in view.subviews_mut() {
            Self::update_view(view);
        }
    }

    fn setup_level(&mut self) {
        self.level = LevelBase::boxed();

        let level = self.level.deref_mut();

        level.setup();

        let square = Image::load(&paths::images().join("square.png"));

        level.add_sprite((0, 0, 1, 1).into());
        level.add_wall((0, 0, 100, 1).into()).set_image(square);
        level.add_wall((20, 0, 1, 100).into()).set_image(square);
        level.add_wall((-20, 0, 1, 100).into()).set_image(square);

        for i in 0..500 {
            level.add_body((0.1 * i as f32, i * 2, 0.5, 0.5).into());
        }
    }

    fn setup_test_view(&mut self) {
        make_view_on::<DebugView>(self.root_view.deref_mut());
        let mut view = make_view_on::<TestView>(self.root_view.deref_mut());

        let mut this = self.level.to_rglica();
        view.dpad.on_up.subscribe(move |_| {
            this.player().jump();
        });

        let mut this = self.level.to_rglica();
        view.dpad.on_left.subscribe(move |_| {
            this.player().go_left();
        });

        let mut this = self.level.to_rglica();
        view.dpad.on_right.subscribe(move |_| {
            this.player().go_right();
        });

        let mut this = self.level.to_rglica();
        view.left_stick
            .on_direction_change
            .subscribe(move |direction| {
                this.player().add_impulse(direction);
            });
    }
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl DesktopInput for TestScreen {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    Event::Moved,
        });
    }

    fn on_mouse_key_pressed(&mut self, _button: glfw::MouseButton, state: Action) {
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    Event::from_state(ButtonState::from_glfw(state)),
        })
    }

    fn on_key_pressed(&mut self, key: Key, action: Action) {
        self.level.on_key_pressed(key, action)
    }
}

#[cfg(any(target_os = "ios", target_os = "android"))]
impl DesktopInput for TestScreen {}

impl Screen for TestScreen {
    fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        self.root_view.calculate_absolute_frame();

        self.setup_level();
        self.setup_test_view();
    }

    fn update(&mut self) {
        GLWrapper::clear();

        let level = self.level.deref_mut();

        level.level_mut().update_physics();
        level.update();

        self.sprites_drawer
            .set_camera_position(&level.player().position());

        for sprite in level.sprites() {
            self.sprites_drawer.draw(sprite);
        }

        TestScreen::update_view(&mut self.root_view);
        self.root_view.calculate_absolute_frame();
        self.ui_drawer.draw(&mut self.root_view);

        self.ui_drawer.reset_viewport();
    }

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.set_frame(Rect::from(size));
        self.sprites_drawer.set_resolution(&size);
        self.sprites_drawer.set_camera_position(&(0, 0).into());
        self.update();
    }
}

impl New for TestScreen {
    fn new() -> TestScreen {
        let mut font_path = ui::DEFAULT_FONT_PATH.lock().unwrap();
        *font_path = paths::fonts().join("SF.otf");
        drop(font_path);
        let assets = Assets::init();
        TestScreen {
            cursor_position: Point::new(),
            assets:          assets.clone(),
            root_view:       ViewBase::boxed(),
            level:           LevelBase::boxed(),
            ui_drawer:       UIDrawer::new(assets.clone()),
            sprites_drawer:  SpritesDrawer::new(assets),
        }
    }
}
