use std::any::Any;

use refs::{Own, Weak};

use crate::{Label, Setup, View};

pub trait __ViewInternalTableData {
    fn __cell_height(&self, index: usize) -> f32;
    fn __variable_height(&self) -> bool;
    fn __number_of_cells(&self) -> usize;
    fn __make_cell(&self, index: usize) -> Own<dyn View>;
    fn __setup_cell(&self, cell: &mut dyn Any, index: usize);
    fn __cell_selected(&mut self, index: usize);
}

pub trait TableData {
    fn cell_height(self: Weak<Self>, index: usize) -> f32;
    fn variable_height(self: Weak<Self>) -> bool;
    fn number_of_cells(self: Weak<Self>) -> usize;
    fn make_cell(self: Weak<Self>, index: usize) -> Own<dyn View>;
    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize);
    fn cell_selected(self: Weak<Self>, index: usize);
}

#[allow(unused_variables)]
impl<T: View + 'static> TableData for T {
    default fn cell_height(self: Weak<Self>, _index: usize) -> f32 {
        50.0
    }

    default fn variable_height(self: Weak<Self>) -> bool {
        false
    }

    default fn number_of_cells(self: Weak<Self>) -> usize {
        0
    }

    default fn make_cell(self: Weak<Self>, index: usize) -> Own<dyn View> {
        Label::new()
    }

    default fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {}

    default fn cell_selected(self: Weak<Self>, index: usize) {}
}
