use refs::Weak;

use crate::View;

pub trait InputView: View {
    fn set_title(&mut self, title: &str);
    fn text(&self) -> &str;
    fn enable_editing(&mut self);
    fn disable_editing(&mut self);
    fn as_input_view(&self) -> Weak<dyn InputView>;
}
