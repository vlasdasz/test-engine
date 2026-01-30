use inspect::ui::ViewRepr;
use ui::{View, ViewData, ViewFrame, ViewSubviews, WeakView};

pub trait ViewToInspect {
    fn view_to_inspect(&self) -> ViewRepr;
}

impl<T: View + ?Sized> ViewToInspect for T {
    fn view_to_inspect(&self) -> ViewRepr {
        ViewRepr {
            label:    self.label().to_string(),
            id:       weak_to_id(self.weak_view()),
            frame:    *self.frame(),
            placer:   self.placer_copy(),
            subviews: self
                .subviews()
                .iter()
                .filter(|v| !v.is_system())
                .map(|v| v.view_to_inspect())
                .collect(),
        }
    }
}

fn weak_to_id(weak_view: WeakView) -> String {
    let raw = weak_view.raw();
    format!(
        "{}:{}",
        hex::encode(raw.addr().to_le_bytes()),
        hex::encode(raw.stamp().to_le_bytes())
    )
}
