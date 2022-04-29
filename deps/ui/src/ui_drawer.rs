use gl_image::Image;
use gm::{flat::Rect, Color};
use rtools::Rglica;

use crate::complex::PathData;

pub trait UIDrawer {
    fn reset_viewport(&self);
    fn fill(&self, rect: &Rect, color: Color);
    fn outline(&self, rect: &Rect, color: Color);
    fn draw_image(&self, image: &Image, rect: &Rect, color: Color, raw_frame: bool);
    fn draw_path(&self, path: &PathData, rect: &Rect);
    fn rglica(&self) -> Rglica<dyn UIDrawer>;
}
