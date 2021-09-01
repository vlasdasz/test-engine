use crate::View;
use gm::Rect;
use proc_macro::New;
use std::cell::RefMut;
use tools::refs::Shared;
use tools::Rglica;

#[derive(New)]
pub struct Layout {
    frame: Rglica<Rect>,
    super_frame: Rglica<Rect>,
}

impl Layout {
    pub fn make(view: &Box<dyn View>) -> Self {
        Self {
            frame: Rglica::from_ref(view.frame()),
            super_frame: Rglica::from_ref(view.super_frame()),
        }
    }
}

impl Layout {
    pub fn br(&mut self) {
        self.frame.origin.x = self.super_frame.size.width - self.frame.size.width;
        self.frame.origin.y = self.super_frame.size.height - self.frame.size.height;
    }

    pub fn distribute_vertically(frame: &Rect, views: &mut [Box<dyn View>]) {
        if views.is_empty() {
            return;
        }

        let mut frames: Vec<&mut Rect> = views.iter_mut().map(|a| a.frame_mut()).collect();
        let height: f32 = frame.height() / frames.len() as f32;
        let width = frame.width();

        for i in 0..frames.len() {
            let frame = &mut frames[i];
            frame.origin.x = 0.0;
            frame.origin.y = i as f32 * height;
            frame.size.width = width;
            frame.size.height = height;
        }
    }
}
