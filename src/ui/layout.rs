use crate::gm::Rect;

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
}
