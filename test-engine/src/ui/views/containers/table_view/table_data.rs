use std::any::Any;

use refs::Own;
use ui::{Container, Label, Setup, View};

use crate::ui::CellRegistry;

pub trait TableData {
    fn cell_height(&self, index: usize) -> f32;
    fn variable_height(&self) -> bool;
    fn number_of_cells(&self) -> usize;
    fn make_cell(&self, index: usize) -> Own<dyn View>;
    fn setup_cell(&self, cell: &mut dyn Any, index: usize);
    fn cell_selected(&self, index: usize);
    fn setup_cell2(&self, index: usize, registry: &mut CellRegistry) -> Own<dyn View>;
}

#[allow(unused_variables)]
impl<T: View + 'static> TableData for T {
    default fn cell_height(&self, _index: usize) -> f32 {
        50.0
    }

    default fn variable_height(&self) -> bool {
        false
    }

    default fn number_of_cells(&self) -> usize {
        0
    }

    default fn make_cell(&self, index: usize) -> Own<dyn View> {
        Label::new()
    }

    default fn setup_cell(&self, cell: &mut dyn Any, index: usize) {}

    default fn cell_selected(&self, index: usize) {}

    default fn setup_cell2(&self, index: usize, registry: &mut CellRegistry) -> Own<dyn View> {
        Container::new()
    }
}
