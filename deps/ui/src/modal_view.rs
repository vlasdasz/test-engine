use dispatch::{from_main, on_main};
use gm::{color::WHITE, flat::Size};
use refs::{Own, Weak};
use vents::OnceEvent;

use crate::{TouchStack, UIManager, View, ViewData, ViewFrame, view::ViewSubviews};

pub trait ModalView<In = (), Out: 'static = ()>: 'static + View + Default {
    fn make_modal(view: Self) -> Weak<Self> {
        let mut view = Own::new(view);
        view.set_z_position(UIManager::MODAL_Z_OFFSET);
        view.set_color(WHITE);
        let size = Self::modal_size();
        let weak = view.weak();
        TouchStack::push_layer(weak.weak_view());
        UIManager::root_view().add_subview_to_root(view);
        weak.place().size(size.width, size.height).center();
        weak
    }

    fn prepare_modally() -> Weak<Self> {
        Self::make_modal(Self::default())
    }

    fn prepare_modally_with_input(input: In) -> Weak<Self> {
        let view = Self::prepare_modally();
        view.setup_input(input);
        view
    }

    fn show_modally_with_input(input: In, callback: impl FnOnce(Out) + 'static + Send)
    where
        In: 'static + Send,
        Out: Send, {
        on_main(move || {
            let weak = Self::prepare_modally_with_input(input);
            weak.modal_event().val(callback);
        });
    }

    #[allow(async_fn_in_trait)]
    async fn show_modally_async(input: In) -> Out
    where
        In: 'static + Send,
        Out: Send, {
        from_main(|| Self::prepare_modally_with_input(input).modal_event().val_async())
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
