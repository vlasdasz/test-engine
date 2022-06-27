#[cfg(desktop)]
use glfw::Action;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyEvent {
    Release,
    Press,
    Repeat,
}

#[cfg(desktop)]
impl From<Action> for KeyEvent {
    fn from(action: Action) -> Self {
        match action {
            Action::Release => Self::Release,
            Action::Press => Self::Press,
            Action::Repeat => Self::Repeat,
        }
    }
}
