use gm::flat::{Point, Size};
use refs::Rglica;
use wgpu::RenderPass;
use winit::event::{ElementState, MouseButton, Touch};

use crate::{render::wgpu_drawer::WGPUDrawer, WGPUApp};

pub trait App {
    fn window_ready(&mut self);
    fn update(&mut self);
    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer);
    fn resize(&mut self, position: Point, size: Size<u32>);
    fn mouse_moved(&mut self, position: Point) -> bool;
    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool;
    fn touch_event(&mut self, touch: Touch) -> bool;
    fn set_wgpu_app(&mut self, app: Rglica<WGPUApp>);
}
