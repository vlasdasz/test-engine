use netrun::Function;

use crate::interface::test_game_view::Node;

#[derive(Default, Clone, Debug)]
pub struct MenuEntry {
    pub label: &'static str,
    action:    Function<(), ()>,
    enabled:   bool,
}

impl MenuEntry {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            action: Function::default(),
            enabled: true,
        }
    }

    pub fn action<Ret>(mut self, mut action: impl FnMut() -> Ret + Send + 'static) -> Self {
        self.action = Function::new(move |()| {
            action();
        });
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn run(&self) {
        self.action.call(());
    }
}

impl From<MenuEntry> for Node<MenuEntry> {
    fn from(value: MenuEntry) -> Self {
        Self::empty(value)
    }
}
