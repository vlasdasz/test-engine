use std::fmt::Debug;

use refs::Weak;

use crate::{text_field_constraint::TextFieldConstraint, ToLabel};

pub trait Labeled: Debug {
    fn text(&self) -> &str;
    fn set_text(&mut self, text: &dyn ToLabel);

    fn title(&self) -> &str;
    fn set_title(&mut self, title: &dyn ToLabel);

    fn set_constraint(&mut self, cons: Option<TextFieldConstraint>);

    fn enable_editing(&mut self);
    fn disable_editing(&mut self);

    fn labeled(&self) -> Weak<dyn Labeled>;
}
