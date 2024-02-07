#[derive(Debug)]
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

    pub fn is_control(&self, control: ControlButton) -> bool {
        match &self.button {
            KeyboardButton::Letter(_) => false,
            KeyboardButton::Control(c) => c == &control,
        }
    }

    pub fn uppercase(&mut self) {
        if let KeyboardButton::Letter(ref mut c) = &mut self.button {
            *c = c.to_ascii_uppercase();
        }
    }

    pub fn char(&self) -> Option<char> {
        match self.button {
            KeyboardButton::Letter(char) => char.into(),
            KeyboardButton::Control(_) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ControlButton {
    Ctrl,
    Alt,
    Del,
    Shift,
    Escape,
    Backspace,
    Unknown,
}

#[derive(Debug)]
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
