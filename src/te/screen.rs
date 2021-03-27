use crate::gm::{Size, Rect, Color, Point};
use crate::te::{TEUIDrawer, Assets};
use crate::ui::View;
use crate::gl_wrapper::gl_drawer::{Updatable, MouseButton, ButtonState};
use crate::ui::input::Touch;
use crate::ui::input::touch::Event;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Screen {
    cursor_position: Point,
    root_view: View,
    ui_drawer: TEUIDrawer
}

impl Screen {
    fn on_touch(&mut self, touch: Touch) {
        log!(touch)
    }
}

impl Updatable for Screen {

    fn new() -> Screen {
        let assets = Assets::init();
        let ui_drawer = TEUIDrawer::new(assets);
        Screen { cursor_position: Point::new(), root_view: View::new(), ui_drawer }
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

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.set_frame(Rect::from_size(&size));
    }

    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    fn on_mouse_key_pressed(&mut self, button: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state)
        })
    }

    fn update(&mut self) {
        self.ui_drawer.draw_view(&mut self.root_view);
    }
}
