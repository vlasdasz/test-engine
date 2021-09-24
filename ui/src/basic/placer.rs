use gm::Rect;
use proc_macro::New;
use tools::Rglica;

use crate::View;

#[derive(New)]
pub struct Placer {
    view:        Rglica<dyn View>,
    frame:       Rglica<Rect>,
    super_frame: Rglica<Rect>,
}

impl Placer {
    pub fn make(view: &mut (dyn View + 'static)) -> Self {
        Self {
            view:        Rglica::from_ref(view),
            frame:       Rglica::from_ref(view.frame()),
            super_frame: Rglica::from_ref(view.super_frame()),
        }
    }
}

impl Placer {
    pub fn at_center(&mut self) {
        self.frame.origin.x = self.super_frame.width() / 2.0 - self.frame.width() / 2.0;
        self.frame.origin.y = self.super_frame.height() / 2.0 - self.frame.height() / 2.0;
    }

    pub fn br(&mut self) {
        self.frame.origin.x = self.super_frame.size.width - self.frame.size.width;
        self.frame.origin.y = self.super_frame.size.height - self.frame.size.height;
    }

    pub fn distribute_vertically(&mut self) {
        let views = self.view.subviews_mut();

        if views.is_empty() {
            return;
        }

        let mut frames: Vec<&mut Rect> = views.iter_mut().map(|a| a.frame_mut()).collect();
        let height: f32 = self.frame.height() / frames.len() as f32;
        let width = self.frame.width();

        for (i, frame) in frames.iter_mut().enumerate() {
            frame.origin.x = 0.0;
            frame.origin.y = i as f32 * height;
            frame.size.width = width;
            frame.size.height = height;
        }
    }
}
