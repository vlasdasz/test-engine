use gm::flat::{IntSize, Point};
use wgpu::RenderPass;
use winit::event::{ElementState, MouseButton};

use crate::wgpu_drawer::WGPUDrawer;

pub trait App {
    fn window_ready(&mut self);
    fn update(&mut self);
    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer);
    fn resize(&mut self, size: IntSize);
    fn mouse_moved(&mut self, position: Point) -> bool;
    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool;
}
