use std::sync::{Mutex, MutexGuard, OnceLock};

use nonempty::NonEmpty;
use refs::Weak;

use crate::{touch_layer::TouchLayer, UIManager, View};

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

    fn get() -> MutexGuard<'static, Self> {
        STACK.get_or_init(Self::init).lock().unwrap()
    }
}

impl TouchStack {
    fn layer_for(&mut self, view: Weak<dyn View>) -> &mut TouchLayer {
        let mut view_stack = vec![];

        view_stack.push(view.label.clone());

        let mut sup = view.superview;

        while sup.is_ok() {
            view_stack.push(sup.label.clone());
            sup = sup.superview;
        }

        for layer in self.stack.iter_mut().rev() {
            for label in &view_stack {
                if layer.root_name() == label {
                    return layer;
                }
            }
        }

        unreachable!("Failed to found view touch layer")
    }
}

impl TouchStack {
    pub fn touch_views() -> Vec<Weak<dyn View>> {
        Self::get().stack.last().views()
    }

    pub fn enable_for(view: Weak<dyn View>, priority: bool) {
        Self::get().layer_for(view).add(view, priority)
    }

    pub fn disable_for(view: Weak<dyn View>) {
        Self::get().layer_for(view).remove(view)
    }

    pub fn push_layer(view: Weak<dyn View>) {
        Self::get().stack.push(view.into())
    }

    pub fn pop_layer(view: Weak<dyn View>) {
        let pop = Self::get().stack.pop().unwrap();
        assert_eq!(
            pop.root_addr(),
            view.addr(),
            "Inconsistent pop_touch_view call. Expected: {} got: {}",
            pop.root_name(),
            view.label
        );
    }

    pub fn root_name() -> String {
        Self::get().stack.last().root_name().to_string()
    }

    pub fn dump() -> String {
        let mut result = String::new();

        for layer in &Self::get().stack {
            result += &format!("Layer: {}\n", layer.root_name());
            for view in layer.views() {
                if view.is_null() {
                    continue;
                }
                result += &format!("View: {}\n", view.label);
            }
            result += "\n";
        }

        result
    }
}
