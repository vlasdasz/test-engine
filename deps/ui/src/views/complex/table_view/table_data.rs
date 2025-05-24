use std::any::Any;

use refs::{Own, Weak};

use crate::{Label, Setup, View};

pub trait __ViewInternalTableData {
    fn __cell_height(&self) -> f32;
    fn __number_of_cells(&self) -> usize;
    fn __make_cell(&self) -> Own<dyn View>;
    fn __setup_cell(&self, cell: &mut dyn Any, index: usize);
    fn __cell_selected(&mut self, #[allow(unused_variables)] index: usize);
}

pub trait TableData {
    fn cell_height(self: Weak<Self>) -> f32;
    fn number_of_cells(self: Weak<Self>) -> usize;
    fn make_cell(self: Weak<Self>) -> Own<dyn View>;
    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize);
    fn cell_selected(self: Weak<Self>, #[allow(unused_variables)] index: usize);
}

#[allow(unused_variables)]
impl<T: View + 'static> TableData for T {
    default fn cell_height(self: Weak<Self>) -> f32 {
        50.0
    }

    default fn number_of_cells(self: Weak<Self>) -> usize {
        0
    }

    default fn make_cell(self: Weak<Self>) -> Own<dyn View> {
        Label::new()
    }

    default fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {}

    default fn cell_selected(self: Weak<Self>, index: usize) {}
}
