use std::rc::Rc;

#[cfg(desktop)]
use gl_wrapper::gl_events::GlEvents;
#[cfg(desktop)]
use glfw::{Action, Key};
use gm::flat::Point;
use rtools::{platform::Platform, IntoF32, Rglica, ToRglica};
use sprites::SpritesDrawer;
#[cfg(desktop)]
use ui::input::TouchEvent;
use ui::{basic::RootView, Touch, ViewFrame, ViewSubviews, ViewTouch};

use crate::{
    debug_view::DebugView, main_view::MainView, sprites_drawer::TESpritesDrawer, ui_drawer::TEUIDrawer,
    Keymap,
};

pub struct UILayer {
    pub sprites_drawer: Box<dyn SpritesDrawer>,
    pub drawer:         TEUIDrawer,

    pub ui_cursor_position: Point,
    pub cursor_position:    Point,
    pub root_view:          Box<RootView>,
    pub debug_view:         Rglica<DebugView>,
    pub view:               Rglica<dyn MainView>,

    pub keymap: Rc<Keymap>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f64,

    scale: f32,
}

impl UILayer {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            sprites_drawer: TESpritesDrawer::new(),
            drawer:         TEUIDrawer::new(),

            ui_cursor_position: Default::default(),
            cursor_position:    Default::default(),
            root_view:          RootView::new(),
            debug_view:         Default::default(),
            view:               Default::default(),
            keymap:             Default::default(),
            fps:                Default::default(),
            prev_time:          Default::default(),
            frame_time:         Default::default(),
            scale:              1.0,
        })
    }
}

impl UILayer {
    pub fn on_touch(&mut self, mut touch: Touch) {
        // trace!("{:?}", touch);
        let level_touch = touch;
        if Platform::DESKTOP {
            touch.position = self.ui_cursor_position;
        } else {
            touch.position /= self.scale;
        }
        if !self.root_view.check_touch(&mut touch) {
            self.view.pass_touch_to_level(level_touch)
        }
        self.root_view.remove_scheduled();
    }

    pub fn set_view<T: MainView + 'static>(&mut self) {
        if self.view.is_ok() {
            self.view.remove_from_superview();
        }
        let mut view: Box<dyn MainView> = T::boxed();
        self.view = view.to_rglica();
        view.set_ui(self.to_rglica());
        view.set_sprites_drawer(self.sprites_drawer.to_rglica());
        self.root_view.add_subview(view);
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: impl IntoF32) {
        self.scale = scale.into_f32();
        self.drawer.set_scale(self.scale);
        self.root_view.set_frame(self.drawer.window_size / self.scale);
    }

    pub fn add_debug_view(&mut self) {
        self.debug_view = self.root_view.add_view();
    }
}

#[cfg(desktop)]
impl UILayer {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.ui_cursor_position = position / self.scale;
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    TouchEvent::Moved,
        })
    }

    fn on_mouse_click(&mut self, _button: glfw::MouseButton, state: Action) {
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    ui::input::MouseButtonState::from_glfw(state).into(),
        })
    }

    fn on_key_pressed(&mut self, key: &str) {
        self.keymap.check(&key);
        if self.view.level().is_ok() {
            self.view.level().on_key_pressed(&key);
        }
    }

    pub fn setup_events(&mut self) {
        let ev = GlEvents::get();

        ev.key_pressed.set(self, |this, a| {
            let key = a.0;
            let action = a.1;

            if action != Action::Press {
                return;
            }

            let key = key.get_name().unwrap_or_else({
                || {
                    match key {
                        Key::Space => " ",
                        _ => "unknown",
                    }
                    .into()
                }
            });

            this.on_key_pressed(&key);

            ui::input::UIEvents::get()
                .key_pressed
                .trigger((key, action.into()));
        });

        ev.mouse_click.set(self, |this, a| this.on_mouse_click(a.0, a.1));

        ev.cursor_moved.set(self, |this, a| this.on_cursor_moved(a))
    }
}
