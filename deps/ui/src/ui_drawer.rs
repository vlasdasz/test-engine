use gm::{flat::Rect, Color};
use rtools::Weak;

use crate::{DrawMode, PathData, View};

pub trait UIDrawer {
    fn reset_viewport(&self);
    fn fill(&self, rect: &Rect, color: &Color);
    fn outline(&self, rect: &Rect, color: &Color);
    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>);
    fn draw_round_border(&self, view: &mut dyn View);
    fn update(&self, view: &mut dyn View);
    fn draw(&self, view: &mut dyn View);
    fn rglica(&self) -> Weak<dyn UIDrawer>;
}
