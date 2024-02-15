use gm::flat::{Point, Size};
use refs::Rglica;
use wgpu::RenderPass;
use winit::event::{ElementState, MouseButton};

use crate::{render::wgpu_drawer::WGPUDrawer, WGPUApp};

pub trait App {
    fn window_ready(&mut self);
    fn update(&mut self);
    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer);
    fn resize(&mut self, size: Size<u32>);
    fn mouse_moved(&mut self, position: Point) -> bool;
    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool;
    fn set_wgpu_app(&mut self, app: Rglica<WGPUApp>);
}
