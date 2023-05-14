use std::fmt::Debug;

use async_trait::async_trait;
use gm::flat::Size;
use refs::{Own, ToWeak, Weak};
use tokio::sync::oneshot::{Receiver, Sender};

use crate::{view::ViewSubviews, UIManager, View};

#[async_trait]
pub trait ModalView<In = (), Out: Default + Debug = ()>: View {
    fn show_modally(self: Own<Self>, input: In) -> Receiver<Out>
    where Self: 'static + Sized {
        let size = Self::modal_size();
        let mut weak = self.weak();
        UIManager::root_view().add_subview(self);
        weak.setup_input(input);
        weak.place.center().size(size.width, size.height);
        UIManager::push_touch_view(weak.weak_view());
        weak.recv()
    }

    fn hide_modal(mut self: Weak<Self>) {
        self.remove_from_superview();
        self.send().send(self.result()).unwrap();
    }

    fn modal_size() -> Size;

    fn send(&mut self) -> Sender<Out>;
    fn recv(&mut self) -> Receiver<Out>;

    fn setup_input(self: Weak<Self>, _: In) {}

    fn result(self: Weak<Self>) -> Out {
        Out::default()
    }
}
