use std::fmt::Debug;

use gm::flat::Size;
use refs::Own;
use ui::View;

pub trait CollectionData: Debug {
    fn number_of_cells(&self) -> usize;
    fn cell_for_index(&self, index: usize) -> Own<dyn View>;
    fn size_for_index(&self, index: usize) -> Size;
    fn cell_selected(&mut self, _index: usize) {}
}

#[macro_export]
macro_rules! collection_data {
    ($source:ident) => {{
        use std::ops::DerefMut;
        ($source.deref_mut() as &mut dyn CollectionData).weak()
    }};
}
