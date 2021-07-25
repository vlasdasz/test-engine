use crate::gl_wrapper::{Buffer, BufferConfig, GLWrapper};
use crate::gm::flat::PointsPath;
use crate::gm::{Color, Rect, Size};
use crate::image::Image;
use crate::te::Assets;
use crate::tools::platform::Platform;
use crate::ui::complex::path_data::DrawMode;
use crate::ui::complex::{DrawingView, PathData};
use crate::ui::view::View;
use crate::GLC;
use std::rc::Rc;
use tools::array_view::ArrayView;
use tools::new;
use tools::refs::Shared;

pub struct UIDrawer {
    pub assets: Rc<Assets>,
    pub window_size: Size,
}

impl UIDrawer {
    pub fn new(assets: Rc<Assets>) -> UIDrawer {
        UIDrawer {
            assets,
            window_size: new(),
        }
    }

    pub fn set_size(&mut self, size: &Size) {
        self.window_size = size.clone();
    }
}

impl UIDrawer {
    pub fn draw(&self, view: Shared<dyn View>) {
        if let Some(image) = view.borrow().image() {
            let view = view.borrow();
            self.draw_image_in_rect(&image, view.absolute_frame(), view.color());
        }

        self.fill_rect(view.borrow().absolute_frame(), &view.borrow().color());

        self.draw_rect(view.borrow().absolute_frame(), &Color::TURQUOISE);

        if let Some(drawing_view) = view.borrow().as_any().downcast_ref::<DrawingView>() {
            for path in &drawing_view.paths {
                self.draw_path_in_rect(path, drawing_view.absolute_frame());
            }
        }

        for view in view.borrow_mut().subviews_mut() {
            self.draw(view.clone())
        }
    }
}

impl UIDrawer {
    pub fn reset_viewport(&self) {
        self.set_viewport(&self.window_size.into());
    }

    fn set_viewport(&self, rect: &Rect) {
        const SCALE: f32 = if Platform::MAC { 2.0 } else { 1.0 };
        GLWrapper::set_viewport(self.window_size.height, &SCALE, rect);
    }
}

impl UIDrawer {
    fn fill_rect(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable();
        self.assets.shaders.ui.set_color(color);
        self.assets.buffers.fullscreen.draw();
    }

    fn draw_rect(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable();
        self.assets.shaders.ui.set_color(color);
        self.assets.buffers.fullscreen_outline.draw();
    }

    fn draw_image_in_rect(&self, image: &Image, rect: &Rect, color: &Color) {
        if image.invalid() {
            return;
        }

        if rect.size.is_negative() {
            return;
        }

        if image.is_monochrome() {
            self.assets.shaders.ui_monochrome.enable();
            self.assets.shaders.ui_monochrome.set_color(color);
        } else {
            self.assets.shaders.ui_texture.enable();
        }

        self.set_viewport(rect);
        image.bind();
        self.assets.buffers.fullscreen_image.draw();
    }
}

impl UIDrawer {
    pub fn draw_path_in_rect(&self, path: &PathData, rect: &Rect) {
        self.set_viewport(rect);
        self.assets.shaders.ui_path.enable();
        self.assets.shaders.ui_path.set_color(&path.color);
        self.assets.shaders.ui_path.set_size(&rect.size);
        path.buffer.draw();
    }

    pub fn initialize_path_data(path: PointsPath, color: Color, draw_mode: DrawMode) -> PathData {
        #[cfg(any(target_os = "ios", target_os = "android"))]
        use gles31_sys::GL_LINE_STRIP;

        let buffer = Buffer::make(
            &BufferConfig::_2,
            ArrayView::from_vector(&path.points),
            None,
            GLC!(LINE_STRIP), //draw_mode.to_gl(),
        );

        PathData {
            buffer,
            path,
            color,
            draw_mode,
        }
    }
}
