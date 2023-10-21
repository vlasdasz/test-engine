use dispatch::{from_main, on_main};
use gm::{flat::Size, Color};
use refs::{Own, Weak};
use vents::Event;

use crate::{view::ViewSubviews, TouchStack, UIManager, View, ViewData};

pub trait ModalView<In = (), Out: 'static = ()>: 'static + View + Default {
    fn prepare_modally(input: In) -> Weak<Self> {
        let mut view = Own::<Self>::default();
        view.set_color(Color::WHITE);
        let size = Self::modal_size();
        let weak = view.weak();
        TouchStack::push_layer(weak.weak_view());
        UIManager::root_view().add_subview(view);
        weak.setup_input(input);
        weak.place.center().size(size.width, size.height);
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

    #[allow(async_fn_in_trait)]
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
            TouchStack::pop_layer(self.weak_view());
            self.modal_event().trigger(result);
        });
    }

    fn modal_event(&self) -> &Event<Out>;

    fn modal_size() -> Size;

    fn setup_input(self: Weak<Self>, _: In) {}
}
