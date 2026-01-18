use inspect::ui::ViewRepr;
use parking_lot::Mutex;
use test_engine::{
    refs::Weak,
    ui::{
        GRAY, HasText, Label, Setup, TURQUOISE, TextAlignment::Left, UIManager, ViewData, ViewFrame,
        ViewSubviews, ViewTouch, WHITE, view,
    },
};

pub(crate) static SHRINK_SCALE: Mutex<f32> = Mutex::new(0.2);

#[view]
pub struct ViewView {
    #[init]
    label: Label,
}

impl Setup for ViewView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(WHITE);
        self.label.set_text_size(10).set_alignment(Left).place().lrt(0).h(10);
        self.enable_touch();

        self.outline(TURQUOISE);
    }

    fn on_selection_changed(mut self: Weak<Self>, selected: bool) {
        self.set_color(if selected { GRAY } else { WHITE });
    }
}

impl ViewView {
    pub fn set_repr(mut self: Weak<Self>, scale: f32, repr: ViewRepr) {
        let shrink_scale = *SHRINK_SCALE.lock();

        self.cleanup();

        self.label.set_text(&repr.label);

        let frame = (
            repr.frame.x() * scale * shrink_scale / UIManager::scale(),
            repr.frame.y() * scale * shrink_scale / UIManager::scale(),
            repr.frame.width() * scale * shrink_scale / UIManager::scale(),
            repr.frame.height() * scale * shrink_scale / UIManager::scale(),
        );

        self.set_frame(frame);

        for sub in repr.subviews {
            let view = self.add_view::<ViewView>();
            view.set_repr(scale, sub);
        }
    }

    pub fn cleanup(self: Weak<Self>) {
        let mut to_remove = vec![];

        for view in self.subviews() {
            if let Some(view) = view.downcast_view::<Self>() {
                view.cleanup();
                to_remove.push(view);
            }
        }

        for mut v in to_remove {
            v.remove_from_superview();
        }
    }
}
