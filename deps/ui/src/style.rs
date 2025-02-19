use crate::View;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    action: fn(&mut dyn View),
}

impl Style {
    pub const fn new(action: fn(&mut dyn View)) -> Self {
        Self { action }
    }

    pub fn apply(&self, view: &mut dyn View) {
        (self.action)(view);
    }
}
