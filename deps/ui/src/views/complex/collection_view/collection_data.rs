use std::any::Any;

use gm::flat::Size;
use refs::Own;

use crate::{Label, View, view::Setup};

pub trait CollectionData {
    fn number_of_cells(&self) -> usize;
    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize);

    fn size_for_index(&self, _index: usize) -> Size {
        (50, 50).into()
    }

    fn make_cell(&self) -> Own<dyn View> {
        Label::new()
    }

    fn cell_selected(&mut self, _index: usize) {}
}
