use gm::flat::IntSize;
use wgpu::RenderPass;

use crate::wgpu_drawer::WGPUDrawer;

pub trait App {
    fn window_ready(&mut self);
    fn update(&mut self);
    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer);
    fn resize(&mut self, size: IntSize);
}
