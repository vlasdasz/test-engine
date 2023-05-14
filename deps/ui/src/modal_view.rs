use std::fmt::Debug;

use async_trait::async_trait;
use gm::flat::Size;
use refs::{Own, ToWeak, Weak};
use tokio::sync::oneshot::{Receiver, Sender};

use crate::{view::ViewSubviews, UIManager, View};

#[async_trait]
pub trait ModalView<In = (), Out: Default + Debug = ()>: View + Default {
    fn show_modally(input: In) -> Receiver<Out>
    where Self: 'static + Sized {
        let view = Own::<Self>::default();
        let size = Self::modal_size();
        let mut weak = view.weak();
        UIManager::root_view().add_subview(view);
        weak.setup_input(input);
        weak.place.center().size(size.width, size.height);
        UIManager::push_touch_view(weak.weak_view());
        weak.recv()
    }

    fn hide_modal(mut self: Weak<Self>, result: Out) {
        self.remove_from_superview();
        self.send().send(result).unwrap();
    }

    fn modal_size() -> Size;

    fn send(&mut self) -> Sender<Out>;
    fn recv(&mut self) -> Receiver<Out>;

    fn setup_input(self: Weak<Self>, _: In) {}
}
