use dispatch::{from_main, on_main};
use gm::{flat::Size, Color};
use refs::{Own, Weak};
use vents::OnceEvent;

use crate::{view::ViewSubviews, TouchStack, UIManager, View, ViewData, ViewFrame};

pub trait ModalView<In = (), Out: 'static = ()>: 'static + View + Default {
    fn prepare_modally(input: In) -> Weak<Self> {
        let mut view = Own::<Self>::default();
        view.set_z_position(UIManager::MODAL_Z_OFFSET);
        view.set_color(Color::WHITE);
        let size = Self::modal_size();
        let weak = view.weak();
        TouchStack::push_layer(weak.weak_view());
        UIManager::root_view_weak().__add_subview_internal(view, true);
        weak.setup_input(input);
        weak.place().size(size.width, size.height).center();
        weak
    }

    fn show_modally(input: In, callback: impl FnOnce(Out) + 'static + Send)
    where
        In: 'static + Send,
        Out: Send, {
        on_main(move || {
            let weak = Self::prepare_modally(input);
            weak.modal_event().val(callback);
        });
    }

    #[allow(async_fn_in_trait)]
    async fn show_modally_async(input: In) -> Out
    where
        In: 'static + Send,
        Out: Send, {
        from_main(|| Self::prepare_modally(input).modal_event().val_async())
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

    fn modal_event(&self) -> &OnceEvent<Out>;

    fn modal_size() -> Size;

    fn setup_input(self: Weak<Self>, _: In) {}
}
