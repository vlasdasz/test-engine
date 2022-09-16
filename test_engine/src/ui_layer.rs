use std::rc::Rc;

#[cfg(desktop)]
use gl_wrapper::gl_events::GlEvents;
#[cfg(desktop)]
use glfw::{Action, Key};
use gm::flat::Point;
use rtools::{platform::Platform, IntoF32, Rglica, Selectable, ToRglica};
use smart_default::SmartDefault;
use sprites::Level;
#[cfg(desktop)]
use ui::input::TouchEvent;
use ui::{basic::RootView, get_ui_drawer, Touch, View, ViewFrame, ViewSubviews, ViewTouch};

use crate::Keymap;

#[derive(SmartDefault)]
pub struct UILayer {
    pub level: Option<Box<dyn Level>>,

    pub ui_cursor_position: Point,
    pub cursor_position:    Point,
    #[default(RootView::new())]
    pub root_view:          Box<RootView>,
    pub view:               Rglica<dyn View>,

    pub keymap: Rc<Keymap>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f64,

    pub selected_view: Rglica<dyn Selectable>,

    #[default = 1.0]
    scale: f32,
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
            if let Some(level) = &mut self.level {
                level.set_cursor_position(level_touch.position);
                if touch.is_began() {
                    level.add_touch(level_touch.position)
                }
            }
        }
        self.root_view.remove_scheduled();
    }

    pub fn set_view(&mut self, view: Box<dyn View>) {
        if self.view.is_ok() {
            self.view.remove_from_superview();
        }
        self.view = view.to_rglica();
        self.root_view.add_subview(view);
    }

    pub fn set_level(&mut self, level: Box<dyn Level>) {
        self.level = level.into();
        self.level.as_mut().unwrap().setup();
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: impl IntoF32) {
        self.scale = scale.into_f32();
        get_ui_drawer().set_scale(self.scale);
        self.root_view
            .set_frame(*get_ui_drawer().window_size() / self.scale);
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
        if let Some(level) = &mut self.level {
            level.on_key_pressed(&key)
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
