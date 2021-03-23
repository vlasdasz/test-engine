use crate::gm::{ Rect, Color };

#[derive(Debug)]
pub struct View<'a> {
    pub color: Color,

    superview: Option<&'a View<'a>>,

    _frame: Rect,
    _absolute_frame: Rect,
    _needs_layout: bool,

    _subviews: Vec<View<'a>>
}

impl<'a> View<'a> {

    pub fn new() -> View<'a> {
        View {
            _frame: Rect::new(),
            _absolute_frame: Rect::new(),
            color: Color::DEFAULT,
            _needs_layout: true,
            _subviews: vec!(),
            superview: None
        }
    }

    pub fn frame(&self) -> &Rect {
        &self._frame
    }

    pub fn set_frame(&mut self, frame: Rect)  {
        self._frame = frame
    }

    pub fn calculate_absolute_frame(&mut self) {
        self._absolute_frame = self._frame;
        if let Some(superview) = self.superview {
            self._absolute_frame.origin += superview._absolute_frame.origin;
        }
    }

    pub fn add_subview(&mut self, mut view: View<'a>) {

    }

    pub fn subviews(&mut self) -> &mut [View<'a>] {
        &mut self._subviews
    }

}
