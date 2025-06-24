use std::path::PathBuf;

use gm::flat::{Point, Size};
use refs::Rglica;
use wgpu::RenderPass;
use winit::event::{ElementState, KeyEvent, MouseButton, Touch};

use crate::Window;

pub trait WindowEvents {
    fn window_ready(&mut self) {}
    fn update(&mut self) {}
    fn render<'a>(&'a mut self, _pass: &mut RenderPass<'a>) {}
    fn resize(&mut self, _inner_pos: Point, _outer_pos: Point, _inner_size: Size, _outer_size: Size) {}
    fn mouse_moved(&mut self, _position: Point) -> bool {
        false
    }
    fn mouse_event(&mut self, _state: ElementState, _button: MouseButton) -> bool {
        false
    }
    fn mouse_scroll(&mut self, _delta: Point) {}
    fn touch_event(&mut self, _touch: Touch) -> bool {
        false
    }
    fn key_event(&mut self, _event: KeyEvent) {}
    fn set_window(&mut self, _app: Rglica<Window>) {}
    fn dropped_file(&mut self, _path: PathBuf) {}
}
