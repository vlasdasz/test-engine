use inspect::ui::ViewRepr;
use ui::{View, ViewData, ViewFrame, ViewSubviews};

pub trait ViewToInspect {
    fn view_to_inspect(&self) -> ViewRepr;
}

impl<T: View + ?Sized> ViewToInspect for T {
    fn view_to_inspect(&self) -> ViewRepr {
        ViewRepr {
            label:    self.label().to_string(),
            frame:    *self.frame(),
            subviews: self.subviews().iter().map(|v| v.view_to_inspect()).collect(),
        }
    }
}
