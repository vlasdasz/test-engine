use dispatch::{from_main, on_main};
use gm::flat::Size;
use refs::{Own, ToWeak, Weak};
use vents::Event;

use crate::{view::ViewSubviews, UIManager, View};

pub trait ModalView<In = (), Out: 'static = ()>: 'static + View + Default {
    fn prepare_modally(input: In) -> Weak<Self> {
        let view = Own::<Self>::default();
        let size = Self::modal_size();
        let weak = view.weak();
        UIManager::root_view().add_subview(view);
        weak.setup_input(input);
        weak.place.center().size(size.width, size.height);
        UIManager::push_touch_view(weak.weak_view());
        weak
    }

    fn show_modally(input: In, callback: impl FnOnce(Out) + 'static + Send)
    where
        In: 'static + Send,
        Out: Send, {
        on_main(move || {
            let weak = Self::prepare_modally(input);
            weak.modal_event().once(callback);
        })
    }

    async fn show_modally_async(input: In) -> Out
    where
        In: 'static + Send,
        Out: Send, {
        from_main(|| Self::prepare_modally(input).modal_event().once_async())
            .await
            .await
            .unwrap()
    }

    fn hide_modal(mut self: Weak<Self>, result: Out)
    where Out: Send {
        on_main(move || {
            self.remove_from_superview();
            self.modal_event().trigger(result);
        });
    }

    fn modal_event(&self) -> &Event<Out>;

    fn modal_size() -> Size;

    fn setup_input(self: Weak<Self>, _: In) {}
}
