use refs::Weak;

use crate::{HasTitle, View};

pub trait InputView: View + HasTitle {
    fn set_text(&mut self, text: &str);
    fn text(&self) -> &str;
    fn enable_editing(&mut self);
    fn disable_editing(&mut self);
    fn as_input_view(&self) -> Weak<dyn InputView>;
}
