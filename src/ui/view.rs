use crate::gm::{Rect, Color, Point};
use crate::ui::input::Touch;

#[derive(Debug)]
pub struct View {
    pub color: Color,

    _frame: Rect,
    _super_frame: Rect,
    _absolute_frame: Rect,
    _needs_layout: bool,

    _subviews: Vec<View>
}

impl View {
    pub fn new() -> View {
        View {
            _frame: Rect::new(),
            _super_frame: Rect::new(),
            _absolute_frame: Rect::new(),
            color: Color::DEFAULT,
            _needs_layout: true,
            _subviews: vec!()
        }
    }

    pub fn frame(&self) -> &Rect {
        &self._frame
    }

    pub fn absolute_frame(&self) -> &Rect {
        &self._absolute_frame
    }

    pub fn set_frame(&mut self, frame: Rect)  {
        self._frame = frame
    }

    pub fn calculate_absolute_frame(&mut self) {
        for view in self._subviews.iter_mut() {
            view._absolute_frame = view._frame;
            view._absolute_frame.origin += self._absolute_frame.origin;
            view.calculate_absolute_frame();
        }
    }

    pub fn add_subview(&mut self, view: View) {
        self._subviews.push(view)
    }

    pub fn make_subview(&mut self, make: fn (&mut View) -> ()) {
        let mut view = View::new();
        make(&mut view);
        self.add_subview(view);
    }

    pub fn subviews(&mut self) -> &mut [View] {
        &mut self._subviews
    }

}

impl View {

    pub fn contains_global_point(&self, point: &Point) -> bool {
        self._absolute_frame.contains(point)
    }

    pub fn on_touch(&mut self, touch: Touch) {
        log!(touch)
    }
}