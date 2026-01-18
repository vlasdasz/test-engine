use inspect::ui::ViewRepr;
use test_engine::{
    refs::Weak,
    ui::{
        Button, HasText, Label, Setup, TURQUOISE, TextAlignment::Left, UIManager, ViewData, ViewFrame,
        ViewSubviews, view,
    },
};

#[view]
pub struct ViewView {
    #[init]
    button: Button,
    label:  Label,
}

impl Setup for ViewView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text_size(10).set_alignment(Left).place().lrt(0).h(10);
        self.button.place().back();
        self.outline(TURQUOISE);
    }
}

impl ViewView {
    pub fn set_repr(mut self: Weak<Self>, scale: f32, repr: ViewRepr) {
        const SHRINK_SCALE: f32 = 0.8;

        self.cleanup();

        self.label.set_text(&repr.label);

        let frame = (
            repr.frame.x() * scale * SHRINK_SCALE / UIManager::scale(),
            repr.frame.y() * scale * SHRINK_SCALE / UIManager::scale(),
            repr.frame.width() * scale * SHRINK_SCALE / UIManager::scale(),
            repr.frame.height() * scale * SHRINK_SCALE / UIManager::scale(),
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
