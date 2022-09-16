#[cfg(desktop)]
use glfw::Action;

pub struct KeyEvent {
    pub button: KeyboardButton,
    pub state:  KeyState,
}

impl KeyEvent {
    pub fn is_press(&self) -> bool {
        matches!(self.state, KeyState::Press)
    }

    pub fn is_release(&self) -> bool {
        matches!(self.state, KeyState::Release)
    }

    pub fn char(&self) -> Option<char> {
        match self.button {
            KeyboardButton::Letter(char) => char.into(),
            _ => None,
        }
    }
}

pub enum ControlButton {
    Ctrl,
    Alt,
    Del,
    Escape,
    Backspace,
    Unknown,
}

pub enum KeyboardButton {
    Letter(char),
    Control(ControlButton),
}

impl From<char> for KeyboardButton {
    fn from(c: char) -> Self {
        Self::Letter(c)
    }
}

impl From<ControlButton> for KeyboardButton {
    fn from(c: ControlButton) -> Self {
        Self::Control(c)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyState {
    Release,
    Press,
    Repeat,
}

#[cfg(desktop)]
impl From<Action> for KeyState {
    fn from(action: Action) -> Self {
        match action {
            Action::Release => Self::Release,
            Action::Press => Self::Press,
            Action::Repeat => Self::Repeat,
        }
    }
}
