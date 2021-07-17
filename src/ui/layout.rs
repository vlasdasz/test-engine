use crate::gm::Rect;
use crate::ui::View;
use std::cell::RefMut;
use tools::refs::Shared;

pub struct Layout<'a> {
    sup_frame: &'a Rect,
    frame: &'a mut Rect,
}

impl<'a> Layout<'a> {
    pub fn new(sup_frame: &'a Rect, frame: &'a mut Rect) -> Layout<'a> {
        Layout { sup_frame, frame }
    }

    pub fn br(frame: &mut Rect, super_frame: &Rect) {
        frame.origin.x = super_frame.size.width - frame.size.width;
        frame.origin.y = super_frame.size.height - frame.size.height;
    }

    pub fn distribute_vertically(frame: &Rect, views: &mut [Shared<dyn View>]) {
        if views.is_empty() {
            return;
        }

        let mut views: Vec<RefMut<dyn View>> = views.iter_mut().map(|a| a.borrow_mut()).collect();

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
