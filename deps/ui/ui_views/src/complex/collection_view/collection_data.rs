use std::{any::Any, fmt::Debug};

use gm::flat::Size;
use refs::Own;
use ui::View;

pub trait CollectionData: Debug {
    fn number_of_cells(&self) -> usize;
    fn make_cell(&self) -> Own<dyn View>;
    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize);
    fn size_for_index(&self, index: usize) -> Size;
    fn cell_selected(&mut self, _index: usize) {}
}

#[macro_export]
macro_rules! collection_data {
    ($source:ident) => {{
        use std::ops::DerefMut;
        test_engine::refs::weak_from_ref($source.deref_mut() as &mut dyn CollectionData)
    }};
}
