use std::any::Any;

use test_engine::gm::flat::Size;
use ui::{
    refs::{Own, Weak},
    view, Container, SubView, View, ViewData, ViewSetup, ViewSubviews, WithHeader,
};
use ui_views::{collection_data, CollectionData, CollectionView, GLLabel};

#[view]
struct TableTestView {
    table: SubView<CollectionView>,
    label: SubView<GLLabel>,
}

impl ViewSetup for TableTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("Label");
        self.table.data_source = collection_data!(self);
        self.table.reload_data();
    }
}

impl CollectionData for TableTestView {
    fn number_of_cells(&self) -> usize {
        50
    }

    fn make_cell(&self) -> Own<dyn View> {
        Own::<Container>::default()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<Container>().unwrap();

        let mut label = cell.add_view::<GLLabel>();
        label.set_text(format!("Cell: {index}"));
        label.place().center().size(100, 20);
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (0, 60).into()
    }

    fn cell_selected(&mut self, index: usize) {
        self.label.set_text(index);
    }
}
impl WithHeader for TableTestView {
    fn header(&self) -> Weak<dyn View> {
        self.label.weak_view()
    }

    fn main_view(&self) -> Weak<dyn View> {
        self.table.weak_view()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<TableTestView>::start().unwrap()
}
