use std::sync::OnceLock;

use nonempty::NonEmpty;
use parking_lot::{Mutex, MutexGuard};

use crate::{
    UIManager, View, WeakView,
    touch_layer::TouchLayer,
    view::{ViewData, ViewSubviews},
};

static STACK: OnceLock<Mutex<TouchStack>> = OnceLock::new();

pub struct TouchStack {
    stack: NonEmpty<TouchLayer>,
}

impl TouchStack {
    fn init() -> Mutex<Self> {
        Self {
            stack: NonEmpty::new(UIManager::get().root_view.weak_view().into()),
        }
        .into()
    }

    pub fn get() -> MutexGuard<'static, Self> {
        STACK.get_or_init(Self::init).lock()
    }
}

impl TouchStack {
    fn layer_for(&mut self, view: WeakView) -> &mut TouchLayer {
        let mut view_stack = vec![];

        view_stack.push(view.label().to_string());

        let mut sup = view.superview();

        while sup.is_ok() {
            view_stack.push(sup.label().to_string());
            sup = sup.superview();
        }

        for layer in self.stack.iter_mut().rev() {
            for label in &view_stack {
                if layer.root_name() == *label {
                    return layer;
                }
            }
        }

        unreachable!("Failed to found view touch layer")
    }
}

impl TouchStack {
    pub fn touch_views() -> impl Iterator<Item = WeakView> {
        Self::get().stack.last().views().into_iter().rev()
    }

    pub fn enable_for(view: WeakView) {
        Self::get().layer_for(view).add(view);
    }

    pub fn enable_for_low_priority(view: WeakView) {
        Self::get().layer_for(view).add_low_priority(view);
    }

    pub fn disable_for(view: WeakView) {
        Self::get().layer_for(view).remove(view);
    }

    pub fn push_layer(view: WeakView) {
        Self::get().stack.push(view.into());
    }

    pub fn touch_root_name_for(view: WeakView) -> String {
        Self::get().layer_for(view).root_name().to_string()
    }

    pub fn pop_layer(view: WeakView) {
        let pop = Self::get().stack.pop().unwrap();
        assert_eq!(
            pop.root.raw(),
            view.raw(),
            "Inconsistent pop_touch_view call. Expected: {} got: {}",
            pop.root_name(),
            view.label()
        );
    }

    pub fn root_name() -> String {
        Self::get().stack.last().root_name().to_string()
    }

    pub fn clear_freed(&mut self) {
        self.stack.tail.retain(|a| a.root.is_ok());

        for layer in self.stack.iter_mut() {
            layer.clear_freed();
        }
    }

    pub fn dump() -> Vec<Vec<String>> {
        UIManager::free_deleted_views();
        TouchStack::get().clear_freed();

        let mut result = vec![];

        for layer in &Self::get().stack {
            let mut layer_vec = vec![];

            layer_vec.push(format!("Layer: {}", layer.root_name()));

            for view in layer.views() {
                assert!(view.is_ok(), "Null view in touch stack");
                layer_vec.push(view.label().to_string());
            }

            result.push(layer_vec);
        }

        result
    }
}
