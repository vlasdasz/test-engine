use crate::gm::Rect;

pub struct Layout<'a> {
    sup_frame: &'a Rect,
    frame: &'a mut Rect,
}

impl<'a> Layout<'a> {
    pub fn new(sup_frame: &'a Rect, frame: &'a mut Rect) -> Layout<'a> {
        Layout { sup_frame, frame }
    }

    pub fn br(&mut self) {
        self.frame.origin.x = self.sup_frame.size.width - self.frame.size.width;
        self.frame.origin.y = self.sup_frame.size.height - self.frame.size.height;
    }
}
