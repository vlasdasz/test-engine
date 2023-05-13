use async_trait::async_trait;
use gm::flat::Size;
use refs::{Own, Weak};

use crate::{view::ViewSubviews, UIManager, View};

#[async_trait]
pub trait ModalView<T: Default = ()>: View {
    fn show_modally(self: Own<Self>)
    where Self: 'static + Sized {
        let size = Self::modal_size();
        let weak = self.weak_view();
        UIManager::root_view().add_subview(self);
        weak.place.center().size(size.width, size.height);
        UIManager::push_touch_view(weak);
    }
    fn modal_size() -> Size;

    async fn result(self: Weak<Self>) -> T {
        T::default()
    }
}
