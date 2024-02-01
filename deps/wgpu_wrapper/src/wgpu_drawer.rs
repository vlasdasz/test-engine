use anyhow::Result;
use gl_wrapper::path_data::{DrawMode, PathData};
use gm::{flat::Rect, Color};
use wgpu::{Device, Queue, RenderPass, TextureFormat};
use wgpu_text::glyph_brush::Section;

use crate::{colored_image_state::ColoredImageState, image::Image, rect_state::RectState, text::Font};

#[derive(Debug)]
pub struct WGPUDrawer {
    rect_state:          RectState,
    colored_image_state: ColoredImageState,
}

impl WGPUDrawer {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Result<Self> {
        Ok(Self {
            rect_state:          RectState::new(device, texture_format),
            colored_image_state: ColoredImageState::new(device)?,
        })
    }
}

impl WGPUDrawer {
    pub fn fill_rect<'a>(
        &'a self,
        device: &Device,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
    ) {
        self.rect_state.draw(device, render_pass, rect, color);
    }

    pub fn draw_image<'a>(&'a self, render_pass: &mut RenderPass<'a>, image: &'static Image, rect: &Rect) {
        self.colored_image_state.draw(image, rect, render_pass);
    }

    pub fn draw_text(&self, device: &Device, queue: &Queue, section: &Section, font: &'static mut Font) {
        font.brush.queue(device, queue, vec![section]).unwrap()
    }

    pub fn draw_path(
        &self,
        _path: &PathData,
        _rect: &Rect,
        _custom_mode: Option<DrawMode>,
        _priority: usize,
    ) {
        todo!()
    }

    // pub fn draw(&self, _view: &dyn View) {
    //     todo!()
    // }

    pub fn set_root_frame(&mut self, _frame: Rect) {
        todo!()
    }
}
